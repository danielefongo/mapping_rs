use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Path};

use crate::utils::get_into;

pub fn map(attrs: TokenStream, input: TokenStream) -> TokenStream {
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

pub fn map_to(attrs: TokenStream, input: TokenStream) -> TokenStream {
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

pub fn map_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
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
