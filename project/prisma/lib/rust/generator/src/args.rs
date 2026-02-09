use std::collections::BTreeSet;
use std::sync::Arc;

use dmmf::DataModelMetaFormat;
use dmmf::serialization_ast::DmmfInputField;
use dmmf::serialization_ast::TypeLocation;
use generator_shared::extensions::DmmfSchemaExtension;
use generator_shared::extensions::ScalarTypeExtension;
use psl::Schema;
use psl::Validated;
use psl::ValidatedSchema;
use psl::parser_database::ScalarFieldType;
use psl::parser_database::ScalarType;
use query_structure::FieldArity;
use query_structure::walkers::ScalarFieldWalker;
use serde_json::Value;

use super::jsonrpc::GenerateRequest;

#[derive(Debug)]
pub struct Filter {
    pub name: String,
    pub fields: Vec<DmmfInputField>,
}

pub struct GeneratorArgs {
    pub config: psl::Generator,
    pub schema: Arc<Schema<Validated>>,
    pub dmmf: Arc<DataModelMetaFormat>,
    pub scalars: BTreeSet<ScalarType>,
    pub read_params: Vec<Filter>,
    pub write_params: Vec<Filter>,
}

impl GeneratorArgs {
    pub(crate) fn new(
        config: psl::Generator,
        dmmf: Arc<DataModelMetaFormat>,
        schema: Arc<Schema<Validated>>,
    ) -> Self {
        let scalars = Self::generate_scalars(&dmmf);
        let read_params = Self::generate_read_params(&dmmf, &scalars, schema.context());
        let write_params = Self::generate_write_params(&dmmf, &scalars, schema.context());

        Self {
            config,
            schema,
            dmmf,
            scalars,
            read_params,
            write_params,
        }
    }

    pub fn read_filter(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(r#type) => r#type.as_str(),
            ScalarFieldType::Enum(r#enum) => field.db.walk(r#enum).name(),
            ScalarFieldType::Extension(_) | ScalarFieldType::Unsupported(_) => return None,
        };

        let arity = match field.ast_field().arity {
            FieldArity::Required => "",
            FieldArity::Optional => "Nullable",
            FieldArity::List => "List",
        };

        self.read_params
            .iter()
            .find(|param| param.name == format!("{base}{arity}"))
    }

    pub fn write_param(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(r#type) => r#type.as_str(),
            ScalarFieldType::Enum(r#enum) => field.db.walk(r#enum).name(),
            ScalarFieldType::Extension(_) | ScalarFieldType::Unsupported(_) => return None,
        };

        let arity = match field.ast_field().arity {
            FieldArity::Required => "",
            FieldArity::Optional => "Nullable",
            FieldArity::List => "List",
        };

        self.write_params
            .iter()
            .find(|param| param.name == format!("{base}{arity}"))
    }

    fn generate_scalars(dmmf: &DataModelMetaFormat) -> BTreeSet<ScalarType> {
        dmmf.schema
            .input_object_types
            .get("prisma")
            .unwrap()
            .iter()
            .flat_map(|scalar| {
                scalar.fields.iter().flat_map(|field| {
                    field.input_types.iter().filter_map(|input| {
                        matches!(input.location, TypeLocation::Scalar)
                            .then(|| ScalarType::try_from_str(&input.typ, true))
                            .flatten()
                    })
                })
            })
            .collect::<BTreeSet<_>>()
    }

    fn generate_read_params(
        dmmf: &DataModelMetaFormat,
        scalars: &BTreeSet<ScalarType>,
        schema: &ValidatedSchema,
    ) -> Vec<Filter> {
        let mut filters = vec![];

        for scalar in scalars {
            let possible_filters = [
                scalar.to_dmmf_string() + "ListFilter",
                scalar.to_dmmf_string() + "NullableListFilter",
                scalar.to_dmmf_string() + "Filter",
                scalar.to_dmmf_string() + "NullableFilter",
            ];

            filters.extend(possible_filters.iter().filter_map(|filter| {
                let filter_type = dmmf.schema.find_input_type(filter)?.clone();

                let mut s = scalar.as_str().to_string();

                // checking for both is invalid - fields can be list or null but not both
                // TODO: make this more typesafe to correspond with fields
                if filter_type.name.contains("List") {
                    s += "List";
                } else if filter_type.name.contains("Nullable") {
                    s += "Nullable";
                }

                Some(Filter {
                    name: s,
                    fields: filter_type.fields.clone().into_iter().collect(),
                })
            }));
        }

        for enm in schema.db.walk_enums() {
            let possible_filters = [
                "Enum".to_string() + &enm.ast_enum().name.name + "Filter",
                "Enum".to_string() + &enm.ast_enum().name.name + "NullableFilter",
            ];

            filters.extend(possible_filters.iter().filter_map(|filter| {
                let filter_type = dmmf.schema.find_input_type(filter)?.clone();

                let mut name = enm.ast_enum().name.name.clone();

                // checking for both is invalid - fields can be list or null but not both
                // TODO: make this more typesafe to correspond with fields
                if filter_type.name.contains("List") {
                    name += "List";
                } else if filter_type.name.contains("Nullable") {
                    name += "Nullable";
                }

                Some(Filter {
                    name,
                    fields: filter_type.fields.clone().into_iter().collect(),
                })
            }));
        }

        // for i in 0..dml.models.len() {
        //     let m = &dml.models[i];
        //     let p =
        //         match schema.find_input_type(&(m.name.to_string() +
        // "OrderByRelevanceInput")) {             Some(p) => p,
        //             None => continue,
        //         };

        //     let mut methods = vec![];

        //     for field in &p.fields {
        //         if let Some(method) = input_field_as_method(field) {
        //             methods.push(method);
        //         }
        //     }

        //     filters.push(Filter {
        //         name: m.name.clone(),
        //         methods,
        //     });

        //     dml.models[i]
        //         .fields
        //         .push(Field::ScalarField(ScalarField::new(
        //             "relevance",
        //             FieldArity::Optional,
        //             FieldType::Enum(p.name.clone()),
        //         )));
        // }

        filters
    }

    fn generate_write_params(
        dmmf: &DataModelMetaFormat,
        scalars: &BTreeSet<ScalarType>,
        schema: &ValidatedSchema,
    ) -> Vec<Filter> {
        let mut filters = vec![];

        filters.extend(scalars.iter().flat_map(|scalar| {
            if matches!(scalar, ScalarType::Json) {
                return vec![Filter {
                    name: "Json".to_owned(),
                    fields: vec![],
                }];
            }

            let possible_inputs = [
                format!("{}FieldUpdateOperationsInput", scalar.to_dmmf_string()),
                format!("Nullable{}FieldUpdateOperationsInput", scalar.to_dmmf_string()),
            ];

            possible_inputs
                .into_iter()
                .filter_map(|input| {
                    let input_type = dmmf.schema.find_input_type(&input)?;

                    let mut filter_name = scalar.as_str().to_owned();

                    if input_type.name.contains("List") {
                        filter_name += "List";
                    } else if input_type.name.contains("Nullable") {
                        filter_name += "Nullable";
                    }

                    Some(Filter {
                        name: filter_name,
                        fields: input_type
                            .fields
                            .clone()
                            .into_iter()
                            .filter_map(|field| {
                                field.input_types.iter().find(|type_ref| {
                                    matches!(type_ref.location, TypeLocation::Scalar if type_ref.typ != "Null")
                                })?;

                                Some(field)
                            })
                            .collect(),
                    })
                })
                .collect()
        }));

        filters.extend(schema.db.walk_enums().flat_map(|enm| {
            let possible_inputs = [
                format!("Enum{}FieldUpdateOperationsInput", enm.name()),
                format!("NullableEnum{}FieldUpdateOperationsInput", enm.name()),
            ];

            possible_inputs.into_iter().filter_map(move |input| {
                let input_type = dmmf.schema.find_input_type(&input)?.clone();

                let mut name = enm.name().to_string();

                if input_type.name.contains("List") {
                    name += "List";
                } else if input_type.name.contains("Nullable") {
                    name += "Nullable";
                }

                Some(Filter {
                    name,
                    fields: input_type
                        .fields
                        .clone()
                        .into_iter()
                        .filter_map(|field| {
                            field.input_types.iter().find(
                                |inner_input_type| match inner_input_type.location {
                                    TypeLocation::Scalar if inner_input_type.typ != "Null" => true,
                                    TypeLocation::Scalar
                                    | TypeLocation::InputObjectTypes
                                    | TypeLocation::OutputObjectTypes
                                    | TypeLocation::EnumTypes
                                    | TypeLocation::FieldRefTypes => true,
                                },
                            )?;

                            Some(field)
                        })
                        .collect(),
                })
            })
        }));

        filters.extend(schema.db.walk_models().flat_map(|model| {
            model
                .fields()
                .filter_map(|field| {
                    let input_type = dmmf
                        .schema
                        .find_input_type(&format!("{}Update{}Input", model.name(), field.name()))?
                        .clone();

                    let mut fields = vec![];

                    let scalar_name = {
                        let mut scalar_name = None;

                        fields.extend(input_type.fields.clone().into_iter().filter_map(
                            |inner_field| {
                                if inner_field.name == "set" {
                                    for inner_input_type in &inner_field.input_types {
                                        match inner_input_type.location {
                                            TypeLocation::Scalar
                                                if inner_input_type.typ != "null" =>
                                            {
                                                scalar_name =
                                                    Some(inner_input_type.typ.clone() + "List");
                                            }
                                            TypeLocation::Scalar
                                            | TypeLocation::InputObjectTypes
                                            | TypeLocation::OutputObjectTypes
                                            | TypeLocation::EnumTypes
                                            | TypeLocation::FieldRefTypes => {}
                                        }
                                    }
                                }

                                inner_field
                                    .input_types
                                    .iter()
                                    .find(|inner_input_type| match inner_input_type.location {
                                        TypeLocation::Scalar if inner_input_type.typ != "null" => {
                                            true
                                        }
                                        TypeLocation::Scalar
                                        | TypeLocation::InputObjectTypes
                                        | TypeLocation::OutputObjectTypes
                                        | TypeLocation::EnumTypes
                                        | TypeLocation::FieldRefTypes => false,
                                    })
                                    .map(|_| inner_field.clone())
                            },
                        ));

                        scalar_name
                    }?;

                    Some(Filter {
                        name: scalar_name,
                        fields,
                    })
                })
                .collect::<Vec<_>>()
        }));

        filters
    }
}
