pub mod variant;

use syn::Attribute;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DataUnion;
use syn::DeriveInput;
use syn::Fields;
use syn::Generics;
use syn::Ident;
use syn::Member;
use syn::Result;
use syn::Type;
use syn::spanned::Spanned;
use variant::Variant;

#[derive(Clone, Debug)]
pub struct Unresolved;

//
pub trait Attributes: Sized {
    fn from_syn(attributes: &[Attribute]) -> syn::Result<Self>;
}

pub trait Meta<T, TAttributes>: Sized
where
    TAttributes: Attributes,
{
    fn from_syn(data: &T, attributes: &TAttributes) -> syn::Result<Self>;
}

impl Attributes for Unresolved {
    fn from_syn(_attributes: &[Attribute]) -> syn::Result<Self> {
        Ok(Self {})
    }
}

impl<T, TA: Attributes> Meta<T, TA> for Unresolved {
    fn from_syn(_data: &T, _attributes: &TA) -> syn::Result<Self> {
        Ok(Self {})
    }
}

//
#[derive(Debug)]
pub enum Input<'syn> {
    Struct(Struct<'syn>),
    Enum(Enum<'syn>),
    Union(Union<'syn>),
}

impl<'syn> Input<'syn> {
    pub fn from_syn(node: &'syn DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Ok(Input::Struct(Struct::from_syn(node, data)?)),
            Data::Enum(data) => Ok(Input::Enum(Enum::from_syn(node, data)?)),
            Data::Union(data) => Ok(Input::Union(Union::from_syn(node, data)?)),
        }
    }

    pub fn attributes<A: Attributes>(&self) -> syn::Result<A> {
        match self {
            Input::Struct(input) => A::from_syn(&input.node.attrs),
            Input::Enum(input) => A::from_syn(&input.node.attrs),
            Input::Union(_) => todo!(),
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            Input::Struct(input) => input.name,
            Input::Enum(input) => input.name,
            Input::Union(_) => todo!(),
        }
    }

    pub fn generics(&self) -> &Generics {
        match self {
            Input::Struct(input) => input.generics,
            Input::Enum(input) => input.generics,
            Input::Union(_) => todo!(),
        }
    }
}

//
#[derive(Clone, Debug)]
pub enum StructKind {
    Named,
    Unnamed,
    Unit,
}

#[derive(Debug)]
pub struct Struct<'syn, SA = Unresolved, SM = Unresolved, FA = Unresolved, FM = Unresolved> {
    pub node: &'syn DeriveInput,
    pub data: &'syn DataStruct,
    pub kind: StructKind,
    pub name: &'syn Ident,

    pub generics: &'syn Generics,
    pub fields: Vec<Field<'syn, FA, FM>>,

    pub attributes: SA,
    pub meta: SM,
}

impl<'syn> Struct<'syn> {
    pub fn from_syn(node: &'syn DeriveInput, data: &'syn DataStruct) -> Result<Self> {
        let kind = match data.fields {
            Fields::Named(_) => StructKind::Named,
            Fields::Unnamed(_) => StructKind::Unnamed,
            Fields::Unit => StructKind::Unit,
        };

        Ok(Self {
            node,
            data,
            kind,
            name: &node.ident,
            generics: &node.generics,
            fields: Field::from_syn_vec(&data.fields)?,
            attributes: Unresolved,
            meta: Unresolved,
        })
    }

    pub fn resolve<
        SA: Attributes,
        SM: Meta<Struct<'syn, Unresolved, Unresolved, FA, FM>, SA>,
        FA: Attributes,
        FM: Meta<Field<'syn>, FA>,
    >(
        self,
    ) -> syn::Result<Struct<'syn, SA, SM, FA, FM>> {
        let fields = self
            .fields
            .into_iter()
            .map(Field::resolve::<FA, FM>)
            .collect::<syn::Result<Vec<_>>>()?;

        let resolved_fields = Struct {
            node: self.node,
            data: self.data,
            kind: self.kind,
            name: self.name,
            generics: self.generics,
            fields,
            attributes: self.attributes,
            meta: self.meta,
        };

        let attributes = SA::from_syn(&self.node.attrs)?;
        let meta = SM::from_syn(&resolved_fields, &attributes)?;

        Ok(Struct {
            node: resolved_fields.node,
            data: resolved_fields.data,
            kind: resolved_fields.kind,
            name: resolved_fields.name,
            generics: resolved_fields.generics,
            fields: resolved_fields.fields,
            attributes,
            meta,
        })
    }

    pub fn resolve_attributes<NA: Attributes>(&self) -> syn::Result<Struct<'syn, NA, Unresolved>> {
        Ok(Struct {
            node: self.node,
            data: self.data,
            kind: self.kind.clone(),
            name: self.name,
            generics: self.generics,
            fields: self.fields.clone(),
            attributes: NA::from_syn(&self.node.attrs)?,
            meta: self.meta.clone(),
        })
    }

    pub fn resolve_meta<NM: Meta<Self, Unresolved>>(
        &self,
    ) -> syn::Result<Struct<'syn, Unresolved, NM>> {
        let meta = NM::from_syn(self, &Unresolved)?;

        Ok(Struct {
            node: self.node,
            data: self.data,
            kind: self.kind.clone(),
            name: self.name,
            generics: self.generics,
            fields: self.fields.clone(),
            attributes: self.attributes.clone(),
            meta,
        })
    }
}

//
#[derive(Debug)]
pub struct Enum<'syn, A = Unresolved, M = Unresolved> {
    pub node: &'syn DeriveInput,
    pub data: &'syn DataEnum,
    pub name: &'syn Ident,
    pub generics: &'syn Generics,
    pub variants: Vec<Variant<'syn>>,
    pub attributes: A,
    pub meta: M,
}

impl<'syn> Enum<'syn> {
    pub fn from_syn(node: &'syn DeriveInput, data: &'syn DataEnum) -> Result<Self> {
        let variants = data
            .variants
            .iter()
            .map(Variant::from_syn)
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self {
            node,
            data,
            name: &node.ident,
            generics: &node.generics,
            variants,
            attributes: Unresolved,
            meta: Unresolved,
        })
    }

    pub fn resolve<NA: Attributes, NM: Meta<Self, NA>>(self) -> syn::Result<Enum<'syn, NA, NM>> {
        let attributes = NA::from_syn(&self.node.attrs)?;
        let meta = NM::from_syn(&self, &attributes)?;

        Ok(Enum {
            node: self.node,
            data: self.data,
            name: self.name,
            generics: self.generics,
            variants: self.variants,
            attributes,
            meta,
        })
    }

    pub fn resolve_attributes<NA: Attributes>(&self) -> syn::Result<Enum<'syn, NA, Unresolved>> {
        Ok(Enum {
            node: self.node,
            data: self.data,
            name: self.name,
            generics: self.generics,
            variants: self.variants.clone(),

            attributes: NA::from_syn(&self.node.attrs)?,
            meta: self.meta.clone(),
        })
    }

    pub fn resolve_meta<NM: Meta<Self, Unresolved>>(
        self,
    ) -> syn::Result<Enum<'syn, Unresolved, NM>> {
        let meta = NM::from_syn(&self, &Unresolved)?;

        Ok(Enum {
            node: self.node,
            data: self.data,
            name: self.name,
            generics: self.generics,
            variants: self.variants,
            attributes: self.attributes,
            meta,
        })
    }
}

//
#[derive(Debug)]
pub struct Union<'syn, A = Unresolved, M = Unresolved> {
    pub node: &'syn DeriveInput,
    pub data: &'syn DataUnion,
    pub attributes: A,
    pub meta: M,
}

impl<'syn> Union<'syn> {
    pub fn from_syn(node: &'syn DeriveInput, data: &'syn DataUnion) -> Result<Self> {
        Ok(Self {
            node,
            data,
            attributes: Unresolved,
            meta: Unresolved,
        })
    }
}

//
#[derive(Clone, Debug)]
pub struct Field<'syn, A = Unresolved, M = Unresolved> {
    pub node: &'syn syn::Field,
    pub member: Member,
    pub ty: &'syn Type,
    pub attributes: A,
    pub meta: M,
}

impl<'syn> Field<'syn> {
    fn from_syn_vec(fields: &'syn Fields) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Self::from_syn(i, field))
            .collect()
    }

    fn from_syn(index: usize, field: &'syn syn::Field) -> Result<Self> {
        Ok(Field {
            node: field,
            member: match &field.ident {
                Some(ident) => Member::Named(ident.clone()),
                None => Member::Unnamed(syn::Index {
                    index: u32::try_from(index)
                        .map_err(|error| syn::Error::new_spanned(field, error))?,
                    span: field.span(),
                }),
            },
            ty: &field.ty,
            attributes: Unresolved,
            meta: Unresolved,
        })
    }

    pub fn resolve<NA: Attributes, NM: Meta<Self, NA>>(self) -> syn::Result<Field<'syn, NA, NM>> {
        let attributes = NA::from_syn(&self.node.attrs)?;
        let meta = NM::from_syn(&self, &attributes)?;

        Ok(Field {
            node: self.node,
            member: self.member,
            ty: self.ty,
            attributes,
            meta,
        })
    }

    pub fn resolve_attributes<NA: Attributes>(&self) -> syn::Result<Field<'syn, NA, Unresolved>> {
        Ok(Field {
            node: self.node,
            member: self.member.clone(),
            ty: self.ty,
            attributes: NA::from_syn(&self.node.attrs)?,
            meta: self.meta.clone(),
        })
    }

    pub fn resolve_meta<NM: Meta<Self, Unresolved>>(
        &self,
    ) -> syn::Result<Field<'syn, Unresolved, NM>> {
        let meta = NM::from_syn(self, &Unresolved)?;

        Ok(Field {
            node: self.node,
            member: self.member.clone(),
            ty: self.ty,
            attributes: self.attributes.clone(),
            meta,
        })
    }
}
