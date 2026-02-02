mod migration;
mod model_enums;
mod models;
mod module;
mod prisma;

use std::{
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
    path::Path,
    process::Command,
};

use convert_case::{Case, Casing};
use models::generate_models_module;
use module::Module;
use prisma::generate_prisma_module;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use railgun_error::ResultExt;

use super::{
    Generator, GeneratorArgs, Result,
    config::{self, Config},
    error::{ExternalErrorContext, IoErrorContext, StringErrorContext},
};

pub struct RustGenerator;

impl Generator for RustGenerator {
    fn name(&self) -> &'static str {
        "Prisma Client Rust"
    }

    fn default_output(&self) -> &'static str {
        "../src/lib.rs"
    }

    fn generate(&self, args: GeneratorArgs) -> Result<()> {
        let raw_path = &args.config.output.as_ref().unwrap().as_literal().unwrap();
        let output_path = Path::new(raw_path);

        let serialized = serde_json::to_string(&args.config.config).unwrap();
        let config: Config = serde_json::from_str(&serialized)
            .boxed_local()
            .context(ExternalErrorContext {
                reason: "Failed to parse prisma schema config",
            })?;

        match &config.client_format {
            config::ClientFormat::File if output_path.extension().is_none() => {
                return StringErrorContext {
                    reason: "Output path must be a file when using 'client_format = file'",
                }
                .fail();
            }
            config::ClientFormat::Folder if output_path.extension().is_some() => {
                return StringErrorContext {
                    reason: "Output path must be a folder when using 'client_format = folder'",
                }
                .fail();
            }
            config::ClientFormat::File | config::ClientFormat::Folder => {}
        }

        let modules = Self::generate_client(&args)?;

        // We don't care about the error here. If we fail to remove the directory
        // for a permission reason, we'll get the error later down the line when
        // trying to create files.
        //
        // We just want to ideally have a clean slate here.
        remove_dir_all(output_path).context(IoErrorContext); // TODO: ?

        match &config.client_format {
            config::ClientFormat::File => write_file(&modules.flatten(), output_path)?,
            config::ClientFormat::Folder => write_module(&modules, output_path)?,
        }

        Ok(())
    }
}

impl RustGenerator {
    fn generate_client(args: &GeneratorArgs) -> Result<Module> {
        let datamodel = args.schema.context().db.iter_sources().collect::<Vec<_>>().join("\n");
        let datasource = args
            .schema
            .context()
            .configuration
            .datasources
            .first()
            .unwrap()
            .provider
            .clone();

        let migration_metadata = migration::generate_migration_metadata(args)?;
        let enums = model_enums::generate_model_enums(args);

        let mut module = Module::new(
            "client",
            quote! {
                pub use prisma::PrismaClient;

                pub use ::generator_runtime::query::QueryError;
                pub use ::generator_runtime::prisma_value::BigDecimal;

                /// The resolved datamodel provided to us by the prisma cli when
                /// triggering the generator.
                ///
                /// This accounts for the "prismaSchemaFolder" preview feature
                /// that was introduced in 5.15.
                ///
                /// It's needed to reconstruct prisma engine internals at runtime.
                ///
                pub static PRISMA_SCHEMA: &str = #datamodel;

                /// The database provider, used when constructing raw queries to resolve
                /// positional prepared parameters.
                ///
                /// TODO: Compare to the actually resolved provider during runtime. It's
                ///       technically possible to connect using a different database. It
                ///       would not be pretty. Let's provide a nice error when doing so.
                static DATABASE_PROVIDER: &str = #datasource;

                #migration_metadata

                #enums
            },
        );

        module.add_submodule(generate_prisma_module(args)?);
        module.add_submodule(generate_models_module(args));

        Ok(module)
    }
}

fn write_module(module: &Module, path: &Path) -> Result<()> {
    if !module.submodules.is_empty() {
        for submodule in &module.submodules {
            write_module(submodule, &path.join(submodule.name.to_case(Case::Snake)))?;
        }

        let content = &module.content;
        let mod_decls = module.submodules.iter().map(|module| {
            let name = format_ident!("{}", module.name.to_case(Case::Snake));
            quote!(pub mod #name;)
        });

        return write_file(
            &quote! {
                #![allow(dead_code, unused_imports)]

                #(#mod_decls)*

                #content
            },
            &path.join("mod.rs"),
        );
    }

    write_file(&module.content, &path.with_extension("rs"))?;

    Ok(())
}

fn write_file(content: &TokenStream, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).boxed_local().context(ExternalErrorContext {
            reason: "Failed to create parent directory",
        })?;
    }

    let mut file = File::create(path).context(IoErrorContext {})?;

    let pretty_file = syn::parse_file(&content.to_string())
        .boxed_local()
        .context(ExternalErrorContext {
            reason: "Unable to parse generated file.",
        })?;
    let pretty_content = prettyplease::unparse(&pretty_file);

    file.write_all(pretty_content.as_bytes()).context(IoErrorContext {})?;

    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(path.to_str().unwrap()) // We've already validated this
        .output()
        .boxed_local()
        .context(ExternalErrorContext {
            reason: "Failed to run rustfmt over generated prisma module.",
        })?;

    Ok(())
}
