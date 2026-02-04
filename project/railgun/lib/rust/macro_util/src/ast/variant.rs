use std::fmt::Debug;

use syn::Ident;

use super::{Attributes, Field, Meta, Unresolved};

#[derive(Clone, Debug)]
pub struct Variant<'syn, A = Unresolved, M = Unresolved> {
    pub node: &'syn syn::Variant,
    pub name: Ident,
    pub fields: Vec<Field<'syn>>,
    pub attributes: A,
    pub meta: M,
}

impl<'syn> Variant<'syn> {
    pub fn from_syn(variant: &'syn syn::Variant) -> syn::Result<Self> {
        Ok(Variant {
            node: variant,
            name: variant.ident.clone(),
            fields: Field::from_syn_vec(&variant.fields)?,
            attributes: Unresolved,
            meta: Unresolved,
        })
    }

    pub fn resolve<NA: Attributes, NM: Meta<Self, NA>>(self) -> syn::Result<Variant<'syn, NA, NM>> {
        let attributes = NA::from_syn(&self.node.attrs)?;
        let meta = NM::from_syn(&self, &attributes)?;

        Ok(Variant {
            node: self.node,
            name: self.name,
            fields: self.fields,
            attributes,
            meta,
        })
    }

    pub fn resolve_attributes<NA: Attributes>(&self) -> syn::Result<Variant<'syn, NA, Unresolved>> {
        Ok(Variant {
            node: self.node,
            name: self.name.clone(),
            fields: self.fields.clone(),
            attributes: NA::from_syn(&self.node.attrs)?,
            meta: self.meta.clone(),
        })
    }

    pub fn resolve_meta<NM: Meta<Self, Unresolved>>(
        &self,
    ) -> syn::Result<Variant<'syn, Unresolved, NM>> {
        let meta = NM::from_syn(self, &Unresolved)?;

        Ok(Variant {
            node: self.node,
            name: self.name.clone(),
            fields: self.fields.clone(),
            attributes: self.attributes.clone(),
            meta,
        })
    }
}
