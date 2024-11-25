use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldsInfo>,
}

#[allow(dead_code)]
#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(_input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&_input).expect("解析输入失败")
    else {
        panic!("模式匹配失败")
    };

    println!("{:#?}", fields);

    let fields = fields.iter().filter_map(|item| {
        if item.skip {
            return None;
        }

        let ident = item.ident.as_ref()?;

        Some(quote! {
            d.field(stringify!(#ident), &self.#ident);
        })
    });

    quote! {
        impl #generics std::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut d = f.debug_struct(stringify!(#ident));
                #(#fields)*
                d.finish()
            }
        }
    }
}
