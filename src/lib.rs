extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Path, Type};

#[proc_macro_attribute]
pub fn map_struct(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation_from = generate_from_struct_implementation(
        input.clone(),
        input.ident.to_token_stream(),
        target_name.to_token_stream(),
    );

    let implementation_to = generate_from_struct_implementation(
        input.clone(),
        target_name.to_token_stream(),
        input.ident.to_token_stream(),
    );

    quote! {
        #input
        #implementation_from
        #implementation_to
    }
    .into()
}

#[proc_macro_attribute]
pub fn map_struct_to(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation = generate_from_struct_implementation(
        input.clone(),
        input.ident.to_token_stream(),
        target_name.to_token_stream(),
    );

    quote! {
        #input
        #implementation
    }
    .into()
}

#[proc_macro_attribute]
pub fn map_struct_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation = generate_from_struct_implementation(
        input.clone(),
        target_name.to_token_stream(),
        input.ident.to_token_stream(),
    );

    quote! {
        #input
        #implementation
    }
    .into()
}

#[proc_macro_attribute]
pub fn map_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation_from = generate_from_enum_implementation(
        input.clone(),
        input.ident.to_token_stream(),
        target_name.to_token_stream(),
    );

    let implementation_to = generate_from_enum_implementation(
        input.clone(),
        target_name.to_token_stream(),
        input.ident.to_token_stream(),
    );

    quote! {
        #input
        #implementation_from
        #implementation_to
    }
    .into()
}

#[proc_macro_attribute]
pub fn map_enum_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation = generate_from_enum_implementation(
        input.clone(),
        input.ident.to_token_stream(),
        target_name.to_token_stream(),
    );

    quote! {
        #input
        #implementation
    }
    .into()
}

#[proc_macro_attribute]
pub fn map_enum_to(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation = generate_from_enum_implementation(
        input.clone(),
        target_name.to_token_stream(),
        input.ident.to_token_stream(),
    );

    quote! {
        #input
        #implementation
    }
    .into()
}

fn generate_from_enum_implementation(
    input: DeriveInput,
    source_name: TokenStream2,
    target_name: TokenStream2,
) -> TokenStream2 {
    let mut target_match_branches = TokenStream2::new();

    let Data::Enum(data_enum) = input.data else {
        panic!("Enum expected")
    };

    for variant in &data_enum.variants {
        let ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Named(_) => panic!("Not implemented yet"),
            syn::Fields::Unnamed(named) => {
                let rhs = get_into(
                    syn::Ident::new("value", variant.span()),
                    &named.unnamed.first().unwrap().ty,
                );
                target_match_branches.extend(quote_spanned! {variant.span() =>
                   #target_name::#ident(value) => Self::#ident(#rhs),
                })
            }
            syn::Fields::Unit => target_match_branches.extend(quote_spanned! {variant.span() =>
               #target_name::#ident => Self::#ident,
            }),
        }
    }

    quote! {
       impl From<#target_name> for #source_name {
            fn from(value: #target_name) -> Self {
                match value {
                    #target_match_branches
                }
            }
        }
    }
}

fn generate_from_struct_implementation(
    input: DeriveInput,
    source_name: TokenStream2,
    target_name: TokenStream2,
) -> TokenStream2 {
    let mut target_fields = TokenStream2::new();

    let Data::Struct(ref data_struct) = input.data else {
        panic!("Struct expected")
    };

    let fields = match &data_struct.fields {
        syn::Fields::Named(named) => named.named.clone(),
        syn::Fields::Unnamed(_) => panic!("Not implemented yet"),
        syn::Fields::Unit => panic!("Not implemented yet"),
    };

    for field in fields {
        let ident = field.ident.clone().unwrap();
        let into = get_into(ident.clone(), &field.ty);

        target_fields.extend(quote_spanned! {field.span() =>
            #ident: value.#into,
        })
    }

    quote! {
        impl From<#source_name> for #target_name {
            fn from(value: #source_name) -> Self {
                Self {
                    #target_fields
                }
            }
        }
    }
    .into()
}

fn get_into(ident: syn::Ident, ty: &Type) -> TokenStream2 {
    if kind_is(ty, "Vec") {
        quote! { #ident.into_iter().map(Into::into).collect() }
    } else if kind_is(ty, "Option") {
        quote! { #ident.map(Into::into) }
    } else {
        quote! { #ident.into() }
    }
}

fn kind_is(ty: &Type, kind: &str) -> bool {
    match ty {
        Type::Path(typepath) => {
            typepath.qself.is_none()
                && typepath.path.leading_colon.is_none()
                && typepath.path.segments.len() == 1
                && typepath.path.segments.iter().next().unwrap().ident == kind
        }
        _ => false,
    }
}
