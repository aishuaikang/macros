use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).expect("解析输入失败")
    else {
        panic!("模式匹配失败")
    };

    let field = field.map_or_else(
        || {
            if fields.len() == 1 {
                fields.iter().next().unwrap()
            } else {
                panic!("字段属性在存在多个字段时是必需的")
            }
        },
        |field_ident| {
            fields
                .iter()
                .find(|f| f.ident == Some(field_ident.clone()))
                .unwrap()
        },
    );

    let field_ident = field.ident.as_ref().unwrap();
    let field_ty = &field.ty;

    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.#field_ident
            }
        }
    }];

    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#field_ident
                }
            }
        });
    }

    quote! {
        #(#code)*
    }
}
