use proc_macro2::TokenStream as TokenStream2;
use quote::TokenStreamExt;
use syn::DeriveInput;

use crate::utils;

pub fn impl_serde_value(ast: DeriveInput) -> TokenStream2 {
    let name = ast.ident;

    let mut tokens = quote! {
        impl ::slog::SerdeValue for #name {
            fn as_serde(&self) -> &dyn ::erased_serde::Serialize {
                self
            }
            fn to_sendable(&self) -> Box<dyn ::slog::SerdeValue + Send + 'static>  {
                Box::new(self.clone())
            }
        }
    };

    if !utils::contains_named_attr(&ast.attrs, "no_value_impl") {
        let value_impl = quote! {
            impl ::slog::Value for #name {
                fn serialize(&self, _record: &::slog::Record, key: ::slog::Key, ser: &mut dyn ::slog::Serializer) -> ::slog::Result {
                    ser.emit_serde(&key, self)
                }
            }
        };

        tokens.append_all(value_impl);
    }

    tokens
}
