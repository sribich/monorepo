use macro_util::ast::Field;
use macro_util::ast::Meta;
use macro_util::ast::Struct;
use macro_util::ast::Unresolved;
use macro_util::ast::variant::Variant;

use crate::attributes::ContainerAttributes;
use crate::attributes::FieldAttributes;
use crate::attributes::VariantAttributes;

pub struct StructMeta<'syn> {
    pub has_location: bool,
    pub has_source: bool,
    pub has_external_cause: bool,

    pub location_field: Option<Field<'syn, FieldAttributes, FieldMeta>>,
    pub source_field: Option<Field<'syn, FieldAttributes, FieldMeta>>,
    pub error_field: Option<Field<'syn, FieldAttributes, FieldMeta>>,
    pub other_fields: Vec<Field<'syn, FieldAttributes, FieldMeta>>,
}

impl<'syn>
    Meta<Struct<'syn, Unresolved, Unresolved, FieldAttributes, FieldMeta>, ContainerAttributes>
    for StructMeta<'syn>
{
    fn from_syn(
        data: &macro_util::ast::Struct<'syn, Unresolved, Unresolved, FieldAttributes, FieldMeta>,
        _attributes: &ContainerAttributes,
    ) -> syn::Result<Self> {
        let [location_field, source_field, error_field] =
            ["location", "source", "error"].map(|name| {
                data.fields
                    .iter()
                    .find(|field| match &field.node.ident {
                        Some(ident) => ident == name,
                        _ => false,
                    })
                    .cloned()
            });

        let other_fields = data
            .fields
            .iter()
            .filter(|field| {
                if let Some(ident) = &field.node.ident {
                    ident != "location" && ident != "source" && ident != "error"
                } else {
                    false
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(StructMeta {
            has_location: location_field.is_some(),
            has_source: source_field.is_some(),
            has_external_cause: error_field.is_some(),
            location_field,
            source_field,
            error_field,
            other_fields,
        })
    }
}

#[derive(Clone)]
pub struct FieldMeta;

impl<'syn> Meta<Field<'syn>, FieldAttributes> for FieldMeta {
    fn from_syn(_: &Field<'syn>, _attributes: &FieldAttributes) -> syn::Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct VariantMeta<'syn> {
    pub has_location: bool,
    pub has_source: bool,
    pub has_external_cause: bool,

    pub location_field: Option<Field<'syn>>,
    pub source_field: Option<Field<'syn>>,
    pub error_field: Option<Field<'syn>>,
    pub other_fields: Vec<Field<'syn>>,
}

impl<'syn> Meta<Variant<'syn>, VariantAttributes> for VariantMeta<'syn> {
    fn from_syn(data: &Variant<'syn>, attributes: &VariantAttributes) -> syn::Result<Self> {
        let location_field_name = attributes.location_field_name();

        let [location_field, source_field, error_field] = [location_field_name, "source", "error"]
            .map(|name| {
                data.fields
                    .iter()
                    .find(|field| match &field.node.ident {
                        Some(ident) => ident == name,
                        _ => false,
                    })
                    .cloned()
            });

        let other_fields = data
            .fields
            .iter()
            .filter(|field| {
                if let Some(ident) = &field.node.ident {
                    ident != location_field_name && ident != "source" && ident != "error"
                } else {
                    false
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(VariantMeta {
            has_location: location_field.is_some(),
            has_source: source_field.is_some(),
            has_external_cause: error_field.is_some(),
            location_field,
            source_field,
            error_field,
            other_fields,
        })
    }
}
