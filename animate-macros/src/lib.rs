use darling::{FromField, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, LitStr, Path, Type, parse_macro_input};

#[derive(Debug, Default, FromMeta)]
struct AnimateAttr {
    #[darling(default)]
    update: Option<LitStr>,
    #[darling(default, rename = "crate")]
    krate: Option<Path>,
}

#[derive(Debug, Clone, Copy)]
enum RepeatArg {
    Once,
    Times(u32),
    Infinite,
}

impl FromMeta for RepeatArg {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "once" => Ok(Self::Once),
            "infinite" => Ok(Self::Infinite),
            other => Err(darling::Error::unknown_value(other)),
        }
    }

    fn from_value(lit: &syn::Lit) -> darling::Result<Self> {
        match lit {
            syn::Lit::Int(i) => Ok(Self::Times(i.base10_parse()?)),
            syn::Lit::Str(s) => Self::from_string(&s.value()),
            _ => Err(darling::Error::custom("bad repeat")),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(spring, tween))]
struct AnimateField {
    ident: Option<syn::Ident>,
    ty: Type,
    #[darling(default)]
    duration: Option<u64>,
    #[darling(default)]
    delay: Option<u64>,
    #[darling(default)]
    easing: Option<syn::Path>,
    #[darling(default)]
    repeat: Option<RepeatArg>,
    #[darling(default)]
    alternate: Option<bool>,
    #[darling(default)]
    stiffness: Option<f32>,
    #[darling(default)]
    damping: Option<f32>,
    #[darling(default)]
    mass: Option<f32>,
    #[darling(default)]
    epsilon: Option<f32>,
}

#[proc_macro_attribute]
pub fn animate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match darling::ast::NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(darling::Error::from(e).write_errors()),
    };
    let animate_attr = match AnimateAttr::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let input = parse_macro_input!(item as DeriveInput);
    process(input, animate_attr)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn process(input: DeriveInput, attr: AnimateAttr) -> syn::Result<TokenStream2> {
    let krate = attr.krate.unwrap_or_else(|| syn::parse_quote!(animate));

    let method_name = attr
        .update
        .as_ref()
        .map(|s| format_ident!("{}", s.value()))
        .unwrap_or_else(|| format_ident!("advance"));

    let struct_name = &input.ident;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    struct_name,
                    "#[animate] only supports named field structs",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_name,
                "#[animate] only supports structs",
            ));
        }
    };

    let animate_fields = fields
        .iter()
        .map(AnimateField::from_field)
        .collect::<darling::Result<Vec<_>>>()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let mut final_fields = Vec::new();
    let mut params = Vec::new();
    let mut inits = Vec::new();
    let mut advance_calls = Vec::new();

    for (raw, gf) in fields.iter().zip(animate_fields.iter()) {
        let name = gf.ident.as_ref().unwrap();
        let ty = &gf.ty;
        let field_vis = &raw.vis;

        let attrs: Vec<_> = raw
            .attrs
            .iter()
            .filter(|a| !["spring", "tween"].iter().any(|attr| a.path().is_ident(attr)))
            .collect();

        let anim_type = field_anim_type(raw)?;

        match anim_type {
            None => {
                final_fields.push(quote! { #(#attrs)* #field_vis #name: #ty });
                params.push(quote! { #name: #ty });
                inits.push(quote! { #name });
            }
            Some(AnimType::Tween) => {
                final_fields.push(quote! { #(#attrs)* #field_vis #name: #krate::Tween<#ty> });
                params.push(quote! { #name: #ty });
                inits.push(tween_init(&krate, name, gf));
                advance_calls.push(quote! { activity |= self.#name.advance(time); });
            }
            Some(AnimType::Spring) => {
                final_fields.push(quote! { #(#attrs)* #field_vis #name: #krate::Spring<#ty> });
                params.push(quote! { #name: #ty });
                inits.push(spring_init(&krate, name, gf));
                advance_calls.push(quote! { activity |= self.#name.advance(time); });
            }
        }
    }

    Ok(quote! {
        #vis struct #struct_name #impl_generics #where_clause {
            #(#final_fields),*
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn new(#(#params),*) -> Self {
                Self { #(#inits),* }
            }

            pub fn #method_name(&mut self, time: #krate::Time) -> #krate::Activity {
                let mut activity = #krate::Activity::NONE;
                #(#advance_calls)*
                activity
            }
        }
    })
}

enum AnimType {
    Tween,
    Spring,
}

fn field_anim_type(raw: &syn::Field) -> syn::Result<Option<AnimType>> {
    let has_spring = raw.attrs.iter().any(|a| a.path().is_ident("spring"));
    let has_tween = raw.attrs.iter().any(|a| a.path().is_ident("tween"));

    match (has_spring, has_tween) {
        (true, true) => Err(syn::Error::new_spanned(
            raw,
            "field cannot have both #[spring] and #[tween]",
        )),
        (true, false) => Ok(Some(AnimType::Spring)),
        (false, true) => Ok(Some(AnimType::Tween)),
        (false, false) => Ok(None),
    }
}

fn tween_init(krate: &Path, name: &syn::Ident, gf: &AnimateField) -> TokenStream2 {
    let duration = gf.duration.unwrap_or(300);
    let easing = easing_path(krate, gf.easing.as_ref());

    let mut init = quote! {
        #name: #krate::Tween::new(#name)
            .duration(::core::time::Duration::from_millis(#duration))
            .easing(#easing)
    };

    if let Some(delay) = gf.delay {
        init.extend(quote! { .delay(::core::time::Duration::from_millis(#delay)) });
    }

    let repeat = match (gf.repeat, gf.alternate == Some(true)) {
        (Some(RepeatArg::Times(n)), _) => quote! { #krate::Repeat::Times(#n) },
        (Some(RepeatArg::Once), false) | (None, false) => quote! { #krate::Repeat::Once },
        (Some(RepeatArg::Infinite), _) | (_, true) => quote! { #krate::Repeat::Infinite },
    };
    init.extend(quote! { .repeat(#repeat) });

    if gf.alternate == Some(true) {
        init.extend(quote! { .alternate(true) });
    }

    init
}

fn spring_init(krate: &Path, name: &syn::Ident, gf: &AnimateField) -> TokenStream2 {
    let stiffness = gf.stiffness.unwrap_or(200.0);
    let damping = gf.damping.unwrap_or(20.0);
    let mass = gf.mass.unwrap_or(1.0);

    let mut init = quote! {
        #name: #krate::Spring::new(#name)
            .stiffness(#stiffness)
            .damping(#damping)
            .mass(#mass)
    };

    if let Some(epsilon) = gf.epsilon {
        init.extend(quote! { .epsilon(#epsilon) });
    }

    init
}

fn easing_path(krate: &Path, path: Option<&syn::Path>) -> TokenStream2 {
    match path {
        Some(p) if p.leading_colon.is_none() && p.segments.len() == 1 => {
            quote! { #krate::easing::#p }
        }
        Some(p) => quote! { #p },
        None => quote! { #krate::easing::linear },
    }
}
