use proc_macro2::TokenStream as TokenStream2;
use syn::visit;
use syn::{DeriveInput, Ident};

use crate::utils::CollectFields;

pub fn impl_kv(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;

    let mut cf = CollectFields::default();
    visit::visit_derive_input(&mut cf, &ast);
    let fields = cf.fields;

    let field_writes = fields
        .iter()
        .map(|field| (field, field_key(field)))
        .map(|(field, key)| {
            quote! {
                <_ as ::slog::Value>::serialize(&self.#field, _record, #key, ser)
            }
        });

    quote! {
        impl ::slog::KV for #name {
            fn serialize(&self, _record: &::slog::Record, ser: &mut dyn ::slog::Serializer) -> ::slog::Result {
                #(
                    #field_writes?;
                )*

                Ok(())
            }
        }
    }
}

fn field_key(ident: &Ident) -> String {
    ident.to_string()
}
