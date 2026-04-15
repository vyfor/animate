use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

#[derive(Debug, FromField)]
#[darling(attributes(once, cycle, alternate))]
struct AnimateField {
    ident: Option<syn::Ident>,
    ty: Type,
    #[darling(default)]
    duration: Option<u64>,
    #[darling(default)]
    easing: Option<syn::Path>,
    #[darling(default)]
    interp: Option<syn::Path>,
}

#[proc_macro_attribute]
pub fn animate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    process(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn process(input: DeriveInput) -> syn::Result<TokenStream2> {
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

    let animate_fields: Vec<AnimateField> = fields
        .iter()
        .map(AnimateField::from_field)
        .collect::<darling::Result<_>>()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let final_fields = fields.iter().zip(animate_fields.iter()).map(|(raw, gf)| {
        let name = gf.ident.as_ref().unwrap();
        let ty = &gf.ty;
        let field_vis = &raw.vis;

        let mode = raw.attrs.iter().find_map(|a| {
            let p = a.path();
            if p.is_ident("once") { Some(quote!(Once)) }
            else if p.is_ident("cycle") { Some(quote!(Cycle)) }
            else if p.is_ident("alternate") { Some(quote!(Alternate)) }
            else { None }
        });

        let attrs: Vec<_> = raw.attrs.iter().filter(|a| {
            !["once", "cycle", "alternate"].iter().any(|attr| a.path().is_ident(attr))
        }).collect();

        if let Some(mode) = mode {
            quote! { 
                #(#attrs)* #field_vis #name: animate::#mode<#ty, fn(f64) -> f64, fn(&#ty, &#ty, f64) -> #ty> 
            }
        } else {
            quote! { #(#attrs)* #field_vis #name: #ty }
        }
    });

    let mut params = Vec::new();
    let mut inits = Vec::new();

    for (raw, gf) in fields.iter().zip(animate_fields.iter()) {
        let name = gf.ident.as_ref().unwrap();
        let ty = &gf.ty;
        params.push(quote! { #name: #ty });

        let mode = raw.attrs.iter().find_map(|a| {
            let p = a.path();
            if p.is_ident("once") {
                Some(quote!(Once))
            } else if p.is_ident("cycle") {
                Some(quote!(Cycle))
            } else if p.is_ident("alternate") {
                Some(quote!(Alternate))
            } else {
                None
            }
        });

        if let Some(mode) = mode {
            let duration = gf.duration.unwrap_or(0) as f64;
            let easing = easing_path(gf.easing.as_ref());
            let interp = interp_path(ty, gf.interp.as_ref());
            inits.push(quote! {
                #name: animate::#mode::new(#name, #duration, #easing, #interp)
            });
        } else {
            inits.push(quote! { #name });
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
        }
    })
}

fn easing_path(path: Option<&syn::Path>) -> TokenStream2 {
    match path {
        Some(p) if p.leading_colon.is_none() && p.segments.len() == 1 => {
            quote! { animate::easing::#p }
        }
        Some(p) => quote! { #p },
        None => quote! { animate::easing::linear },
    }
}

fn interp_path(ty: &syn::Type, path: Option<&syn::Path>) -> TokenStream2 {
    match path {
        Some(p) if p.leading_colon.is_none() && p.segments.len() == 1 => {
            let module = type_to_module(ty);
            quote! { animate::types::#module::#p }
        }
        Some(p) => quote! { #p },
        None => quote! { <#ty as animate::Lerp>::lerp },
    }
}

fn type_to_module(ty: &syn::Type) -> syn::Ident {
    match ty {
        syn::Type::Path(tp) => {
            let name = tp.path.segments.last().unwrap().ident.to_string();
            let module = match name.as_str() {
                "String" => "string",
                "f64" | "f32" | "usize" | "isize" | "u64" | "i64" | "u32" | "i32" | "u16"
                | "i16" | "u8" | "i8" => "num",
                _ => Box::leak(name.to_lowercase().into_boxed_str()),
            };
            syn::Ident::new(module, proc_macro2::Span::call_site())
        }
        _ => syn::Ident::new("unknown", proc_macro2::Span::call_site()),
    }
}
