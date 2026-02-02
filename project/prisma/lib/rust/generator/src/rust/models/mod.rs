use convert_case::Case;
use generator_shared::{
    casing::cased_ident,
    extensions::{FieldExtension, ModelExtension},
};
use proc_macro2::TokenStream;
use psl::parser_database::ScalarFieldType;
use query_structure::walkers::{FieldWalker, ModelWalker, RefinedFieldWalker};
use quote::quote;

use self::{data::generate_data_field_module, r#where::generate_where_field_module};
use super::{Module, module::FieldModule};
use crate::args::GeneratorArgs;

mod actions;
mod create;
mod data;
mod order_by;
mod pagination;
mod set_params;
mod types;
mod r#where;
mod with_params;

pub fn generate_models_module(args: &GeneratorArgs) -> Module {
    let mut model_module = Module::new(
        "model",
        quote! {
            pub use super::prisma::SortOrder;
        },
    );

    args.schema.context().db.walk_models().for_each(|model| {
        let model_name = model.name();

        //
        let (field_model_data, field_modules) = FieldModule::merge(vec![
            generate_data_field_module(model),
            generate_where_field_module(model, args),
            set_params::generate_field_module(model, args),
            order_by::generate_module(model, args),
            with_params::generate_module(model, args),
        ]);

        let actions = actions::generate_model_actions(model, args);

        let create = create::generate_types(model);
        let types = types::generate_types(model);

        let mut module = Module::new(
            model.name(),
            quote! {
                pub use super::*;
                pub use super::super::prisma::*;

                pub const NAME: &str = #model_name;

                #field_model_data

                #create

                #types

                pub type UniqueArgs = ::generator_runtime::model::FindUniqueArgs<Types>;
                pub type ManyArgs = ::generator_runtime::model::FindManyParams<Types>;

                pub type CountQuery<'db> = ::generator_runtime::model::Count<'db, Types>;
                pub type CreateQuery<'db> = ::generator_runtime::model::Create<'db, Types>;
                pub type CreateUncheckedQuery<'db> = ::generator_runtime::model::CreateUnchecked<'db, Types>;
                pub type CreateManyQuery<'db> = ::generator_runtime::model::CreateMany<'db, Types>;
                pub type FindUniqueQuery<'db> = ::generator_runtime::model::FindUnique<'db, Types>;
                pub type FindManyQuery<'db> = ::generator_runtime::model::FindMany<'db, Types>;
                pub type FindFirstQuery<'db> = ::generator_runtime::model::FindFirst<'db, Types>;
                pub type UpdateQuery<'db> = ::generator_runtime::model::Update<'db, Types>;
                pub type UpdateUncheckedQuery<'db> = ::generator_runtime::model::UpdateUnchecked<'db, Types>;
                pub type UpdateManyQuery<'db> = ::generator_runtime::model::UpdateMany<'db, Types>;
                pub type UpsertQuery<'db> = ::generator_runtime::model::Upsert<'db, Types>;
                pub type DeleteQuery<'db> = ::generator_runtime::model::Delete<'db, Types>;
                pub type DeleteManyQuery<'db> = ::generator_runtime::model::DeleteMany<'db, Types>;

                #actions
            },
        );

        field_modules
            .into_iter()
            .for_each(|field_module| module.add_submodule(field_module));

        model_module.add_submodule(module);
    });

    model_module
}

pub(super) struct RequiredField<'db> {
    pub push_wrapper: TokenStream,
    pub r#type: TokenStream,
    pub inner: FieldWalker<'db>,
}

pub(super) fn get_required_model_fields<'db>(model: &ModelWalker<'db>) -> Option<Vec<RequiredField<'db>>> {
    model
        .fields()
        .filter(|field| {
            field
                .refine()
                .map(|it| match it {
                    RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
                        ScalarFieldType::Enum(_)
                        | ScalarFieldType::Extension(_)
                        | ScalarFieldType::BuiltInScalar(_)
                        | ScalarFieldType::Unsupported(_) => {
                            !model.field_has_relation(scalar_field) && field.is_required()
                        }
                    },
                    RefinedFieldWalker::Relation(_) => field.is_required(),
                })
                .unwrap_or(false)
        })
        .map(|field| {
            Some({
                let r#type = field.refine().and_then(|it| match it {
                    RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
                        ScalarFieldType::Enum(_)
                        | ScalarFieldType::Extension(_)
                        | ScalarFieldType::BuiltInScalar(_)
                        | ScalarFieldType::Unsupported(_) => field.to_tokens(&quote!(super::)),
                    },
                    RefinedFieldWalker::Relation(relation_field) => {
                        let model_name = cased_ident(relation_field.related_model().name(), Case::Snake);

                        Some(quote! { super::#model_name::UniqueWhereParam })
                    }
                })?;

                let push_wrapper = field
                    .refine()
                    .map(|it| match it {
                        RefinedFieldWalker::Scalar(_) => quote! { set },
                        RefinedFieldWalker::Relation(_) => quote! { connect },
                    })
                    .unwrap_or_else(|| quote! {});

                RequiredField {
                    inner: field,
                    push_wrapper,
                    r#type,
                }
            })
        })
        .collect()
}
