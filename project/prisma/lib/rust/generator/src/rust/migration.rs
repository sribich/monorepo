use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use railgun_error::ResultExt;

use crate::{GeneratorArgs, PrismaError, error::IoErrorContext};

pub fn generate_migration_metadata(args: &GeneratorArgs) -> Result<TokenStream, PrismaError> {
    /*
    let schema_path = Path::new(&*args.request.schema_path);

    if !schema_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Could not find a prisma schema file at '{}'",
                schema_path.to_string_lossy()
            ),
        ))
        .context(IoErrorContext {});
    }

    let schema_directory = schema_path.parent();
    */

    // // When using the prisma schema folder feature, the migrations
    // // directory might be up a level from the schema files.
    // //
    // // ATM it seems like this is flawed
    // let migrations_path = [
    //     // schema_directory.and_then(|it| it.parent()),
    //     schema_directory,
    // ]
    // .into_iter()
    // .filter_map(|path| {
    //     path.map(|path| path.join("migrations"))
    // })
    // .find(|path| path.exists())
    // .ok_or_else(|| {
    //     std::io::Error::new(
    //         std::io::ErrorKind::NotFound,
    //         "Could not find the Prisma 'migrations' folder.\nPlease generate initial migrations or create an empty 'migrations' folder in the same folder as the 'schema.prisma' file.",
    //     )
    // })
    // .context(IoErrorContext {})?;

    // let migrations_path = migrations_path
    //     .to_str()
    //     .ok_or_else(|| {
    //         std::io::Error::new(
    //             std::io::ErrorKind::InvalidFilename,
    //             "The path to the 'migrations' folder contains invalid utf8.",
    //         )
    //     })
    //     .context(IoErrorContext {})?;

    // TODO: We need to get this relative to $CARGO_MANIFEST_DIR
    Ok(quote! {
        use ::generator_runtime::migrations::include_dir;

        /// The prisma migrations directory as it exists on disk during a build.
        ///
        /// Required to run migrations at runtime.
        ///
        pub static MIGRATIONS_DIR: &::generator_runtime::migrations::include_dir::Dir =
            &::generator_runtime::migrations::include_dir::include_dir!("$CARGO_MANIFEST_DIR/prisma/migrations");
    })

    // &::generator_runtime::migrations::include_dir::include_dir!(#migrations_path);
}
