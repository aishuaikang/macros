use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let generics = input.generics;
    // println!("generics: {:#?}", generics);

    // println!("ident: {:#?}", ident);
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("EnumFrom can only be derived for enums");
    };
    // println!("variants: {:#?}", variants);
    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        match variant.fields {
            syn::Fields::Unnamed(ref fields_unnamed) => {
                // only one field

                if fields_unnamed.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields_unnamed.unnamed.first().expect("unreachable");
                    let ty = &field.ty;

                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(value: #ty) -> Self {
                                #ident::#variant_ident(value)
                            }
                        }
                    }
                }
            }
            _ => quote! {}, // syn::Fields::Named(fields_named) => quote! {},
                            // syn::Fields::Unit => quote! {},
        }
    });

    // quote return proc-macro2 TokenStream so we need to convert it to proc-macro TokenStream
    quote! {
        #(#from_impls)*
    }
}
