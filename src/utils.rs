use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Type;

pub fn get_into(ident: syn::Ident, ty: &Type) -> TokenStream2 {
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
