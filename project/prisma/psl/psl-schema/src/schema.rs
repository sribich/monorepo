use std::{cell::RefCell, marker::PhantomData, path::PathBuf};

use diagnostics::Diagnostics;

use crate::{
    file::{SchemaFile, SchemaFiles},
    locator::LocatedFiles,
    migrations::Migrations,
    sql::SqlFiles,
};

pub trait SchemaRefiner {
    type From: SchemaRefiner;

    type SchemaContext: Clone;
    type FileContext;

    fn refine_context(&self, from: &Schema<Self::From>) -> Self::SchemaContext;

    fn refine_file(
        &self,
        from: &Schema<Self::From>,
        context: &mut Self::SchemaContext,
        file: &SchemaFile<<Self::From as SchemaRefiner>::FileContext>,
    ) -> Self::FileContext;
}

#[derive(Clone, Debug)]
pub struct DefaultRefiner {}

impl SchemaRefiner for DefaultRefiner {
    type From = DefaultRefiner;

    type FileContext = ();
    type SchemaContext = ();

    fn refine_context(&self, from: &Schema<Self::From>) -> Self::SchemaContext {}

    fn refine_file(
        &self,
        from: &Schema<Self::From>,
        context: &mut Self::SchemaContext,
        file: &SchemaFile<<Self::From as SchemaRefiner>::FileContext>,
    ) -> Self::FileContext {
    }
}

#[derive(Debug)]
pub struct Schema<T = DefaultRefiner>
where
    T: SchemaRefiner,
{
    is_virtual: bool,
    context: T::SchemaContext,
    diagnostics: RefCell<Diagnostics>,
    files: Option<LocatedFiles>,
    schema_files: SchemaFiles<T::FileContext>,
    migrations: Option<Migrations>,
    sql_files: Option<SqlFiles>,

    _marker: PhantomData<T>,
}

impl Schema<DefaultRefiner> {
    pub fn new() -> Schema {
        Self::from_path(None, None)
    }

    pub fn from_path(cwd: Option<PathBuf>, path: Option<PathBuf>) -> Schema {
        let files = LocatedFiles::new(cwd, path);

        Schema {
            is_virtual: false,
            files: Some(files.clone()),
            diagnostics: RefCell::new(Diagnostics::new()),
            context: (),
            schema_files: (&files).into(),
            migrations: (&files).into(),
            sql_files: (&files).into(),
            _marker: PhantomData,
        }
    }

    pub fn from_content<S: AsRef<str>>(content: S) -> Schema {
        let schema = Schema {
            is_virtual: true,
            files: None,
            diagnostics: RefCell::new(Diagnostics::new()),
            context: (),
            schema_files: content.as_ref().into(),
            migrations: None,
            sql_files: None,
            _marker: PhantomData,
        };

        schema
    }
}

impl Default for Schema<DefaultRefiner> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Schema<T>
where
    T: SchemaRefiner,
{
    pub fn context(&self) -> &T::SchemaContext {
        &self.context
    }

    pub fn into_context(self) -> T::SchemaContext {
        self.context
    }

    pub fn diagnostics(&self) -> &RefCell<Diagnostics> {
        &self.diagnostics
    }

    pub fn schema_files(&self) -> &SchemaFiles<T::FileContext> {
        &self.schema_files
    }

    pub fn paths(&self) -> Option<&LocatedFiles> {
        self.files.as_ref()
    }

    pub fn refine<Next>(mut self, refiner: Next) -> Schema<Next>
    where
        Next: SchemaRefiner<From = T>,
    {
        let mut context = refiner.refine_context(&self);

        let schema_files = (&mut self.schema_files).take();
        let schema_files = schema_files
            .into_iter()
            .map(|it| {
                let context = refiner.refine_file(&self, &mut context, &it);
                it.with_context(context)
            })
            .collect::<Vec<_>>();

        Schema {
            is_virtual: self.is_virtual,
            context,
            diagnostics: self.diagnostics.clone(),
            files: self.files.clone(),
            schema_files: SchemaFiles {
                files: schema_files,
            },
            migrations: self.migrations.clone(),
            sql_files: self.sql_files.clone(),
            _marker: PhantomData,
        }
    }
}

/*
/// Render the given diagnostics (warnings + errors) into a String.
    /// This method is multi-file aware.
    pub fn render_diagnostics(&self, diagnostics: &Diagnostics) -> String {
        let mut out = Vec::new();

        for error in diagnostics.errors() {
            let (file_name, source, _) = &self[error.span().file_id];
            error.pretty_print(&mut out, file_name, source.as_str()).unwrap();
        }

        String::from_utf8(out).unwrap()
    }
    */

/*
pub fn list_migrations(&self) {
        let migrations_dir = self.root_dir.join("migrations");

        // // If directory doesn't exist, return an empty array
        if !migrations_dir.try_exists().is_ok_and(identity) {
            panic!("Migration directory does not exist");
            /*
            return {
                        baseDir,
                        lockfile,
                        migrationDirectories: [],
                        shadowDbInitScript,
                      }
            */
        }

        // entries = await fs.readdir(migrationsDirectoryPath, { withFileTypes: true, recursive: false }).catch((_) => [])
        let mut entries = read_dir(migrations_dir)
            .unwrap()
            .into_iter()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| {
                let migration_file = entry.path().join("migration.sql");

                let content = if migration_file.try_exists().is_ok_and(identity) {
                    read_to_string(&migration_file).unwrap()
                } else {
                    return None;
                };

                Some((migration_file, content))
            })
            .filter(|it| it.is_some())
            .map(|it| it.unwrap())
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{:#?}", entries);

        // entries

        /*
        return {
                        baseDir,
                        lockfile,
                        migrationDirectories: sortedMigrations,
                        shadowDbInitScript,
                      }
        */
    }
    */

/*
use crate::FileId;
use diagnostics::Diagnostics;
use psl_ast::ast;
use std::ops::Index;

/// The content is a list of (file path, file source text, file AST).
///
/// The file path can be anything, the PSL implementation will only use it to display the file name
/// in errors. For example, files can come from nested directories.
#[derive(Debug, Clone)]
pub struct Files(pub Vec<(String, psl_ast::SourceFile, ast::SchemaAst)>);

impl Files {
    /// Create a new Files instance from multiple files.
    pub fn new(files: &[(String, psl_ast::SourceFile)], diagnostics: &mut Diagnostics) -> Self {
        let asts = files
            .iter()
            .enumerate()
            .map(|(file_idx, (path, source))| {
                let id = FileId(file_idx as u32);
                let ast = psl_ast::parse_schema(source.as_str(), diagnostics, id);
                (path.to_owned(), source.clone(), ast)
            })
            .collect();
        Self(asts)
    }

    /// Iterate all parsed files.
    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> impl Iterator<Item = (FileId, &String, &psl_ast::SourceFile, &ast::SchemaAst)> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, (path, contents, ast))| (FileId(idx as u32), path, contents, ast))
    }

    /// Iterate all parsed files, consuming the parser database.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = (FileId, String, psl_ast::SourceFile, ast::SchemaAst)> {
        self.0
            .into_iter()
            .enumerate()
            .map(|(idx, (path, contents, ast))| (FileId(idx as u32), path, contents, ast))
    }

    /// Render the given diagnostics (warnings + errors) into a String.
    /// This method is multi-file aware.
    pub fn render_diagnostics(&self, diagnostics: &Diagnostics) -> String {
        let mut out = Vec::new();

        for error in diagnostics.errors() {
            let (file_name, source, _) = &self[error.span().file_id];
            error.pretty_print(&mut out, file_name, source.as_str()).unwrap();
        }

        String::from_utf8(out).unwrap()
    }

    /// Returns the number of files.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<crate::FileId> for Files {
    type Output = (String, psl_ast::SourceFile, ast::SchemaAst);

    fn index(&self, index: crate::FileId) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl<Id> Index<crate::InFile<Id>> for Files
where
    ast::SchemaAst: Index<Id>,
{
    type Output = <ast::SchemaAst as Index<Id>>::Output;

    fn index(&self, index: crate::InFile<Id>) -> &Self::Output {
        &self[index.0].2[index.1]
    }
}

*/
