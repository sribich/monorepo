//! TODO: When creating a struct that has no fields we should emit a `type
//! <name> = Record<string, never>` rather than an empty interface which means
//! any object.
mod ecma;
mod jsdoc;
mod reserved;

use std::borrow::Borrow;

use ecma::is_ecma_ident;
use jsdoc::Jsdoc;
use typegen::Generics;
use typegen::NamedType;
use typegen::cache::ExportIdentifier;
use typegen::cache::TypeCache;
use typegen::datatype::DataType;
use typegen::datatype::NamedDataType;
use typegen::datatype::r#enum::EnumRepr;
use typegen::datatype::r#enum::EnumType;
use typegen::datatype::r#enum::EnumVariantFields;
use typegen::datatype::field::NamedFields;
use typegen::datatype::field::UnnamedFields;
use typegen::datatype::primitive::PrimitiveMeta;
use typegen::datatype::reference::ReferenceType;
use typegen::datatype::r#struct::StructFields;
use typegen::datatype::r#struct::StructType;
use typegen::export::ExportError;
use typegen::export::InvariantErrorContext;
use typegen::export::TypeExporter;
use typegen::internal::Deprecation;

pub fn type_output<T: NamedType>() -> String {
    let mut cache = TypeCache::default();

    let datatype = T::named_datatype(&mut cache, &Generics::Impl);

    Typescript::export((), datatype, &cache).unwrap()
}

pub struct Typescript {}

impl TypeExporter for Typescript {
    type Data = NamedDataType;
    type Options = ();

    fn export(
        _options: Self::Options,
        data: Self::Data,
        cache: &TypeCache,
    ) -> Result<String, ExportError> {
        export_named_type(&data, cache)
    }
}

type TypescriptOutput = Result<String, ExportError>;

fn annotate_type(deprecation: Option<Deprecation>, docs: String, exported_type: String) -> String {
    let mut jsdoc = Jsdoc::default();

    if !docs.is_empty() {
        jsdoc.set_description(docs);
    }

    if let Some(deprecation) = deprecation {
        jsdoc.set_deprecated(deprecation);
    }

    let built_jsdoc = jsdoc.build();

    format!("{built_jsdoc}{exported_type}")
}

fn export_named_type(datatype: &NamedDataType, cache: &TypeCache) -> TypescriptOutput {
    let TypeDefinition { refs, .. } = process_type(cache, datatype.datatype(), "")?;

    if let Some(defs) = refs {
        Ok(annotate_type(
            datatype.deprecation().clone(),
            datatype.doc().clone(),
            defs.join("\n"),
        ))
    } else {
        Ok(String::new())
    }
}

#[derive(Debug)]
pub struct TypeDefinition {
    /// The name ident of the processed type.
    pub name: String,
    /// The target type definition, along with all of its dependencies
    /// that were not already resolved by the cache.
    pub refs: Option<Vec<String>>,
}

// TODO: Put behind internals flag
pub fn process_type(
    cache: &TypeCache,
    datatype: &DataType,
    module: &str,
) -> Result<TypeDefinition, ExportError> {
    match datatype {
        DataType::Unit => Ok(TypeDefinition {
            name: "undefined".to_owned(),
            refs: None,
        }),
        DataType::Primitive(item) => Ok(TypeDefinition {
            name: match item {
                PrimitiveMeta::char | PrimitiveMeta::String => "string".to_string(),
                PrimitiveMeta::i8
                | PrimitiveMeta::i16
                | PrimitiveMeta::i32
                | PrimitiveMeta::i64
                | PrimitiveMeta::i128
                | PrimitiveMeta::isize
                | PrimitiveMeta::u8
                | PrimitiveMeta::u16
                | PrimitiveMeta::u32
                | PrimitiveMeta::u64
                | PrimitiveMeta::u128
                | PrimitiveMeta::usize
                | PrimitiveMeta::f32
                | PrimitiveMeta::f64 => "number".to_string(),
                PrimitiveMeta::bool => "boolean".to_string(),
            },
            refs: None,
        }),
        DataType::Struct(item) => Ok(TypeDefinition {
            name: item.name.to_string(),
            refs: create_struct(cache, item, false, module, false)?,
        }),
        DataType::Enum(item) => Ok(TypeDefinition {
            name: item.name.to_string(),
            refs: create_enum(cache, item, module)?,
        }),
        DataType::List(item) => {
            let inner = process_type(cache, item.inner_type.borrow(), module)?;

            Ok(TypeDefinition {
                name: format!("{}[]", inner.name),
                refs: inner.refs,
            })
        }
        DataType::Tuple(item) => {
            let (names, refs) = item
                .elements
                .iter()
                .map(|item| process_type(cache, item, module))
                .collect::<Result<Vec<_>, ExportError>>()?
                .into_iter()
                .map(|item| (item.name, item.refs))
                .collect::<(Vec<_>, Vec<_>)>();

            let refs = refs.into_iter().flatten().flatten().collect::<Vec<_>>();

            Ok(TypeDefinition {
                name: format!("[{}]", names.join(",")),
                refs: Some(refs),
            })
        }
        DataType::Optional(item) => {
            let real_type = process_type(cache, item, module)?;

            Ok(TypeDefinition {
                name: format!("{} | undefined", real_type.name),
                refs: real_type.refs,
            })
        }
        DataType::Reference(ReferenceType { name, id, generics }) => {
            let mut reference = match generics.as_slice() {
                [] => TypeDefinition {
                    name: name.to_string(),
                    refs: None,
                },
                generics => {
                    let generic_types: (Vec<_>, Vec<_>) = generics
                        .iter()
                        .map(|(_, datatype)| {
                            let TypeDefinition { name, refs } =
                                process_type(cache, datatype, module)?;

                            Ok((name, refs))
                        })
                        .collect::<Result<Vec<(_, _)>, ExportError>>()?
                        .into_iter()
                        .unzip();

                    let generic_idents = generic_types.0.join(", ");
                    let generic_refs = generic_types
                        .1
                        .into_iter()
                        .flatten()
                        .flatten()
                        .collect::<Vec<_>>();

                    let generic_refs = if generic_refs.is_empty() {
                        None
                    } else {
                        Some(generic_refs)
                    };

                    TypeDefinition {
                        name: format!("{name}<{generic_idents}>"),
                        refs: generic_refs,
                    }
                }
            };

            if !cache.is_exported(id)
                && let Some(datatype) = cache.get(id)
            {
                if let Some(mut exported_refs) =
                    process_type(cache, datatype.datatype(), module)?.refs
                {
                    match reference.refs {
                        Some(refs) => {
                            reference.refs = Some(refs).map(|mut inner| {
                                inner.append(&mut exported_refs);
                                inner
                            });
                        }
                        None => {
                            reference.refs = Some(exported_refs);
                        }
                    }
                }

                // cache.set_exported(id, ExportIdentifier::new(module));
            } else {
                // We want to re-process the type to populate module dependencies
                if let Some(datatype) = cache.get(id) {
                    process_type(cache, datatype.datatype(), module)?;
                }
            }

            Ok(reference)
        }
        DataType::Generic(generic) => Ok(TypeDefinition {
            name: generic.0.to_string(),
            refs: None,
        }),
    }
}

fn create_struct(
    cache: &TypeCache,
    dt: &StructType,
    as_fields: bool,
    module: &str,
    for_flatten: bool,
) -> Result<Option<Vec<String>>, ExportError> {
    let StructType {
        id,
        fields,
        name,
        generics,
        ..
    } = dt;

    if cache.is_exported(id) && cache.contains(id) {
        cache.add_export_dependency(id, module.to_owned());
        return Ok(None);
    }

    cache.set_exported(id, ExportIdentifier::new(*id, module));

    let generics = if generics.is_empty() {
        String::new()
    } else {
        format!("<{}>", generics.join(", "))
    };

    let mut items = vec![];
    let mut definition = match fields {
        StructFields::Unit => {
            format!("export type {name}{generics} = Record<string, never>")
        }
        StructFields::Unnamed(UnnamedFields { fields }) => {
            let array_inner = fields
                .iter()
                .filter_map(|field| field.opt.as_ref())
                .map(|datatype| {
                    let TypeDefinition {
                        name: typename,
                        refs,
                    } = process_type(cache, datatype, module)?;

                    if let Some(mut nested_type) = refs {
                        items.append(&mut nested_type);
                    }

                    let typedef = if let DataType::Optional(_) = datatype {
                        format!("({typename} | null)")
                    } else {
                        let generics = datatype
                            .generics()
                            .map(|generics| {
                                let idents = generics
                                    .iter()
                                    .map(|it| it.0.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ");

                                if idents.is_empty() {
                                    String::new()
                                } else {
                                    format!("<{idents}>")
                                }
                            })
                            .unwrap_or_default();

                        format!("{typename}{generics}")
                    };

                    Ok(typedef)
                })
                .collect::<Result<Vec<_>, ExportError>>()?;

            let len = array_inner.len();
            let array_inner = array_inner.join(", ");

            // This is a newtype
            if len == 1 {
                format!("export type {name}{generics} = {array_inner}")
            } else {
                format!("export type {name}{generics} = [{array_inner}]")
            }
        }
        StructFields::Named(NamedFields { fields }) => {
            let interface_fields = fields
                .iter()
                .filter_map(|field| Some((field.0.clone(), field.1.clone(), field.1.opt.as_ref()?)))
                .map(|(name, field, datatype)| {
                    if field.flatten {
                        fn f(
                            cache: &TypeCache,
                            datatype: &DataType,
                            items: &mut Vec<String>,
                            module: &str,
                        ) -> Result<String, ExportError> {
                            match datatype {
                                DataType::Struct(it) => {
                                    let mut flattened =
                                        create_struct(cache, it, true, module, true)?
                                            .unwrap_or(vec![]);
                                    let len = flattened.len();

                                    if len >= 1 {
                                        let last = Vec::split_off(&mut flattened, len - 1);
                                        items.append(&mut flattened);

                                        return Ok(last.join("\n"));
                                    }

                                    // prob broke
                                    // return Ok("".to_owned());
                                    todo!();
                                }
                                DataType::Primitive(_) => todo!(),
                                DataType::Enum(_) => todo!(),
                                DataType::List(_) => todo!(),
                                DataType::Tuple(_) => todo!(),
                                DataType::Optional(_) => todo!(),
                                DataType::Reference(it) => {
                                    let existing = cache.get(&it.id);

                                    if let Some(dt) = existing {
                                        return f(cache, dt.datatype(), items, module);
                                    }

                                    todo!();
                                }
                                DataType::Generic(_) => todo!(),
                                DataType::Unit => todo!(),
                            }
                        }

                        let res = f(cache, datatype, &mut items, module);
                        return res;
                    }

                    let TypeDefinition {
                        name: typename,
                        refs,
                    } = process_type(cache, datatype, module)?;

                    if let Some(mut nested_type) = refs {
                        items.append(&mut nested_type);
                    }

                    let field_name = match is_ecma_ident(name.as_ref()) {
                        true => name.to_string(),
                        false => format!("'{name}'"),
                    };

                    let typedef = if let DataType::Optional(_) = datatype {
                        format!("    {field_name}?: {typename}")
                    } else {
                        format!("    {field_name}: {typename}")
                    };

                    Ok(typedef)
                })
                .collect::<Result<Vec<_>, ExportError>>()?
                .join("\n");

            if as_fields {
                interface_fields
            } else if interface_fields.is_empty() {
                format!("type {name}{generics} = Record<string, never>")
            } else {
                format!("export interface {name}{generics} {{\n{interface_fields}\n}}")
            }
        }
    };

    definition.push('\n');

    // Flattened structs should not be output
    if for_flatten {
        items.push(definition.clone());
        cache.set_export_content(id, String::new());
    } else {
        cache.set_export_content(id, definition);
    }

    Ok(Some(items))
}

fn create_enum(
    cache: &TypeCache,
    EnumType {
        id,
        name,
        generics,
        variants,
        repr,
    }: &EnumType,
    module: &str,
) -> Result<Option<Vec<String>>, ExportError> {
    if cache.is_exported(id) && cache.contains(id) {
        cache.add_export_dependency(id, module.to_owned());
        return Ok(None);
    }

    cache.set_exported(id, ExportIdentifier::new(*id, module));

    let generic_idents = generics
        .iter()
        .map(|generic| generic.0.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let generic_idents = if generic_idents.is_empty() {
        String::new()
    } else {
        format!("<{generic_idents}>")
    };

    let mut result = vec![];

    let items = variants
        .iter()
        .filter(|it| !it.1.skip)
        .map(|(name, variant)| match (repr, &variant.inner) {
            (EnumRepr::External, EnumVariantFields::Unit) => Ok(format!(r#""{}""#, name.clone())),
            (EnumRepr::External, EnumVariantFields::Named(NamedFields { fields })) => {
                let items = fields
                    .iter()
                    .filter(|&(_, field)| field.opt.is_some())
                    .map(|(name, field)| (name, field.opt.as_ref().unwrap()))
                    .map(|(name, field)| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(format!(r#""{name}": {typename}"#))
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                Ok(format!(r#"{{ "{name}": {{ {items} }} }}"#))
            }
            (EnumRepr::External, EnumVariantFields::Unnamed(UnnamedFields { fields })) => {
                let items = fields
                    .iter()
                    .filter_map(|field| field.opt.as_ref())
                    .map(|field| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(typename)
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                // Newtype
                if fields.len() == 1 {
                    Ok(format!(r#"{{ "{name}": {items} }}"#))
                } else {
                    Ok(format!(r#"{{ "{name}": [ {items} ] }}"#))
                }
            }
            (EnumRepr::Adjacent { tag, .. }, EnumVariantFields::Unit) => {
                Ok(format!(r#""{tag}": "{name}""#))
            }
            (
                EnumRepr::Adjacent { tag, content },
                EnumVariantFields::Named(NamedFields { fields }),
            ) => {
                let items = fields
                    .iter()
                    .filter(|&(_, field)| field.opt.is_some())
                    .map(|(name, field)| (name, field.opt.as_ref().unwrap()))
                    .map(|(name, field)| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(format!(r#""{name}": {typename}"#))
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                Ok(format!(
                    r#"{{ "{tag}": "{name}", "{content}": {{ {items} }} }}"#
                ))
            }
            (
                EnumRepr::Adjacent { tag, content },
                EnumVariantFields::Unnamed(UnnamedFields { fields }),
            ) => {
                let items = fields
                    .iter()
                    .filter_map(|field| field.opt.as_ref())
                    .map(|field| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(typename)
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                // Newtype
                if fields.len() == 1 {
                    Ok(format!(r#"{{ "{tag}": "{name}", {content}: {items} }}"#))
                } else {
                    Ok(format!(r#"{{ "{tag}": "{name}", {content}: [{items}] }}"#))
                }
            }
            (EnumRepr::Internal { .. }, EnumVariantFields::Unit) => todo!(),
            (EnumRepr::Internal { .. }, EnumVariantFields::Named(_)) => todo!(),
            (EnumRepr::Internal { tag }, EnumVariantFields::Unnamed(tuple)) => {
                if tuple.fields.len() > 1 {
                    return InvariantErrorContext {
                        msg: "Unnamed tuples cannot be tagged".to_owned(),
                    }
                    .fail();
                }

                // assert!((tuple.fields.len() <= 1), "Unnamed tuples cannot be tagged");

                let items = tuple
                    .fields
                    .iter()
                    .filter_map(|field| field.opt.as_ref())
                    .map(|field| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(typename)
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                Ok(format!(r#"({{ {tag}: "{name}" }} & {items})"#))
            }
            (EnumRepr::Untagged, EnumVariantFields::Unit) => todo!(),
            (EnumRepr::Untagged, EnumVariantFields::Named(NamedFields { fields })) => {
                let items = fields
                    .iter()
                    .filter(|&(_, field)| field.opt.is_some())
                    .map(|(name, field)| (name, field.opt.as_ref().unwrap()))
                    .map(|(name, field)| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(format!(r#""{name}": {typename}"#))
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                Ok(format!("{{ {items} }}"))
            }
            (EnumRepr::Untagged, EnumVariantFields::Unnamed(UnnamedFields { fields })) => {
                let items = fields
                    .iter()
                    .filter_map(|field| field.opt.as_ref())
                    .map(|field| {
                        let TypeDefinition {
                            name: typename,
                            refs,
                        } = process_type(cache, field, module)?;

                        if let Some(mut nested_type) = refs {
                            result.append(&mut nested_type);
                        }

                        Ok(typename)
                    })
                    .collect::<Result<Vec<_>, ExportError>>()?
                    .join(",");

                // Newtype
                if fields.len() == 1 {
                    Ok(items)
                } else {
                    Ok(format!("[{items}]"))
                }
            } //            EnumRepr::External => format!("{{ {}: {} }}", name, process_type(cache,
              // variant.inner)),            EnumRepr::Adjacent { tag, content } =>
              // todo!(),            EnumRepr::Internal { tag } => todo!(),
              //            EnumRepr::Untagged => todo!(),
        })
        .collect::<Result<Vec<_>, ExportError>>()?;

    let item = format!(
        "export type {}{} = {}\n",
        name,
        generic_idents,
        if items.is_empty() {
            "Record<never, never>".to_owned()
        } else {
            items.join(" | ")
        }
    );

    cache.set_export_content(id, item);
    // result.push(item);

    Ok(Some(result))
}
