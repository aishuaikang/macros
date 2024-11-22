use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(debug))]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldsInfo>,

    #[darling(default)]
    skip: bool,
}
#[allow(dead_code)]
#[derive(Debug, FromField)]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
}

pub(crate) fn process_auto_debug(_input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(_fields),
        skip,
    } = AutoDebugInfo::from_derive_input(&_input).expect("解析输入失败")
    else {
        panic!("模式匹配失败")
    };

    quote! {
        impl #generics std::fmt::Debug for #ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if #skip {
                    return Ok(());
                }

                todo!()
            }
        }
    }
}
