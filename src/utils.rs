use syn::{Attribute, Field, Ident};
use syn::visit::{self, Visit};

pub fn contains_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.segments[0].ident.to_string() == name)
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

        if !contains_attr(&field.attrs, "skip") {
            self.fields.push(name.clone());
        }

        visit::visit_field(self, field);
    }
}
