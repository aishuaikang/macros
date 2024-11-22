mod auto_debug;
mod auto_deref;
mod enum_from;
mod enum_from_darling;

use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;

// for enum, we'd like to generate From impls for each variant(对于枚举，我们希望为每个变体生成 From 实现)
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from_darling(input).into()
}

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    auto_deref::process_auto_deref(input).into()
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    auto_debug::process_auto_debug(input).into()
}
