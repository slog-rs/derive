use syn::visit::{self, Visit};
use syn::{Attribute, Field, Ident, Meta, NestedMeta};

const SLOG_ATTRIBUTE: &str = "slog";

/// Looks for the bare attribute, `#[slog($name)]`.
pub fn contains_named_attr(attrs: &[Attribute], name: &str) -> bool {
    slog_attributes(attrs)
        .into_iter()
        .any(|meta| meta.name() == name)
}

/// Get the contents of all `#[slog(...)]` attributes.
pub fn slog_attributes(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .into_iter()
        .filter_map(|attr| attr.interpret_meta())
        .filter_map(|meta| match meta {
            Meta::List(list) => Some(list),
            _ => None,
        })
        .filter(|meta_list| meta_list.ident == SLOG_ATTRIBUTE)
        .flat_map(|ml| ml.nested)
        .filter_map(|nested| match nested {
            NestedMeta::Meta(m) => Some(m),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Default)]
pub struct CollectFields {
    pub fields: Vec<Ident>,
}

impl<'a> Visit<'a> for CollectFields {
    fn visit_field(&mut self, field: &Field) {
        let name = field
            .ident
            .as_ref()
            .expect("You can't use this derive on a tuple struct");

        if !contains_named_attr(&field.attrs, "skip") {
            self.fields.push(name.clone());
        }

        visit::visit_field(self, field);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{self, DeriveInput};

    #[test]
    fn find_the_skip_attribute() {
        let src = "#[slog(skip)] struct Foo {}";
        let foo: DeriveInput = syn::parse_str(src).unwrap();
        let attrs = &foo.attrs;

        assert!(contains_named_attr(attrs, "skip"));
    }
}
