use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

#[derive(Debug, FromField)]
#[darling(attributes(grease))]
struct GreaseField {
    ident: Option<syn::Ident>,
    ty: Type,
    #[darling(default)]
    duration: Option<u64>,
    #[darling(default)]
    easing: Option<syn::Path>,
}

#[proc_macro_attribute]
pub fn grease(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
                    "#[grease] only supports named field structs",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_name,
                "#[grease] only supports structs",
            ));
        }
    };

    let grease_fields: Vec<GreaseField> = fields
        .iter()
        .map(GreaseField::from_field)
        .collect::<darling::Result<_>>()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    let final_fields = fields.iter().zip(grease_fields.iter()).map(|(raw, gf)| {
        let name = gf.ident.as_ref().unwrap();
        let ty = &gf.ty;
        let field_vis = &raw.vis;
        let attrs: Vec<_> = raw
            .attrs
            .iter()
            .filter(|a| !a.path().is_ident("grease"))
            .collect();

        if gf.duration.is_some() {
            quote! { #(#attrs)* #field_vis #name: grease::Grease<#ty> }
        } else {
            quote! { #(#attrs)* #field_vis #name: #ty }
        }
    });

    let mut params = Vec::new();
    let mut inits = Vec::new();

    for (_, gf) in fields.iter().zip(grease_fields.iter()) {
        let name = gf.ident.as_ref().unwrap();
        let ty = &gf.ty;
        params.push(quote! { #name: #ty });

        match gf.duration {
            Some(duration) => {
                let duration = duration as f64;
                let easing = easing_path(gf.easing.as_ref());
                inits.push(quote! { #name: grease::Grease::new(#name, #duration, #easing) });
            }
            None => inits.push(quote! { #name }),
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
            quote! { grease::easing::#p }
        }
        Some(p) => quote! { #p },
        None => quote! { grease::easing::linear },
    }
}
