extern crate proc_macro;

mod enum_mapping;
mod struct_mapping;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn map_struct(attrs: TokenStream, input: TokenStream) -> TokenStream {
    struct_mapping::map(attrs, input)
}

#[proc_macro_attribute]
pub fn map_struct_to(attrs: TokenStream, input: TokenStream) -> TokenStream {
    struct_mapping::map_to(attrs, input)
}

#[proc_macro_attribute]
pub fn map_struct_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    struct_mapping::map_from(attrs, input)
}

#[proc_macro_attribute]
pub fn map_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    enum_mapping::map(attrs, input)
}

#[proc_macro_attribute]
pub fn map_enum_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    enum_mapping::map_from(attrs, input)
}

#[proc_macro_attribute]
pub fn map_enum_to(attrs: TokenStream, input: TokenStream) -> TokenStream {
    enum_mapping::map_to(attrs, input)
}
