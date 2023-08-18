extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Path};

#[proc_macro_attribute]
pub fn map_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let target_name = parse_macro_input!(attrs as Path);

    let implementation_from = generate_from_implementation(
        input.clone(),
        input.ident.to_token_stream(),
        target_name.to_token_stream(),
    );

    let implementation_to = generate_from_implementation(
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

    let implementation = generate_from_implementation(
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

    let implementation = generate_from_implementation(
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

fn generate_from_implementation(
    input: DeriveInput,
    source_name: TokenStream2,
    target_name: TokenStream2,
) -> TokenStream2 {
    let mut target_match_branches = TokenStream2::new();

    let Data::Enum(data_enum) = input.data else {
        panic!("Enum expected")
    };

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Named(_) => panic!("Named variants is not implemented yet"),
            syn::Fields::Unnamed(_) => {
                target_match_branches.extend(quote_spanned! {variant.span() =>
                   #target_name::#variant_name(value) => Self::#variant_name(value.into()),
                })
            }
            syn::Fields::Unit => target_match_branches.extend(quote_spanned! {variant.span() =>
               #target_name::#variant_name => Self::#variant_name,
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
