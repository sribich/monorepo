use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::locator::LocatedFiles;

#[derive(Clone, Debug)]
pub struct SchemaFiles<T> {
    pub files: Vec<SchemaFile<T>>,
}

impl<T> SchemaFiles<T> {
    pub fn iter(&self) -> impl Iterator<Item = &SchemaFile<T>> {
        self.files.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SchemaFile<T>> {
        self.files.iter_mut()
    }

    pub fn into_iter(self) -> impl Iterator<Item = SchemaFile<T>> {
        self.files.into_iter()
    }

    pub fn take(&mut self) -> Self {
        SchemaFiles {
            files: std::mem::take(&mut self.files),
        }
    }

    /*
    // Iterate all parsed files.
    pub fn iter(&self) -> impl Iterator<Item = (FileId, &String, &psl_ast::SourceFile, &ast::SchemaAst)> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, (path, contents, ast))| (FileId(idx as u32), path, contents, ast))
    }
     */

    /*
    /// Iterate all parsed files, consuming the parser database.
    pub fn into_iter(self) -> impl Iterator<Item = (FileId, String, psl_ast::SourceFile, ast::SchemaAst)> {
        self.0
            .into_iter()
            .enumerate()
            .map(|(idx, (path, contents, ast))| (FileId(idx as u32), path, contents, ast))
    }
     */
}

impl From<&LocatedFiles> for SchemaFiles<()> {
    fn from(value: &LocatedFiles) -> SchemaFiles<()> {
        let files = value
            .schema_files
            .iter()
            .map(|it| SchemaFile::from_path(it.clone()))
            .collect::<Vec<_>>();

        SchemaFiles { files }
    }
}

impl From<&LocatedFiles> for Option<SchemaFiles<()>> {
    fn from(value: &LocatedFiles) -> Option<SchemaFiles<()>> {
        Some(value.into())
    }
}

impl From<&str> for SchemaFiles<()> {
    fn from(value: &str) -> Self {
        SchemaFiles {
            files: vec![SchemaFile::from_content(value.to_owned())],
        }
    }
}

#[derive(Clone, Debug)]
pub enum SchemaFile<T = ()> {
    Real {
        id: usize,
        path: PathBuf,
        content: String,
        context: T,
    },
    Virtual {
        id: usize,
        content: String,
        context: T,
    },
}

impl SchemaFile<()> {
    pub fn from_path(path: PathBuf) -> SchemaFile<()> {
        let content = read_to_string(&path).unwrap();

        SchemaFile::Real {
            id: 0,
            path,
            content,
            context: (),
        }
    }

    pub fn from_content(content: String) -> SchemaFile<()> {
        SchemaFile::Virtual {
            id: 0,
            content,
            context: (),
        }
    }
}

impl<T> SchemaFile<T> {
    pub fn context(&self) -> &T {
        match self {
            SchemaFile::Real { context, .. } | SchemaFile::Virtual { context, .. } => context,
        }
    }

    pub fn with_context<Next>(self, context: Next) -> SchemaFile<Next> {
        match self {
            SchemaFile::Real {
                id, path, content, ..
            } => SchemaFile::Real {
                id,
                path,
                content,
                context,
            },
            SchemaFile::Virtual { id, content, .. } => SchemaFile::Virtual {
                id,
                content,
                context,
            },
        }
    }

    pub fn path(&self) -> &str {
        match self {
            SchemaFile::Real { path, .. } => path.to_str().unwrap(),
            SchemaFile::Virtual { .. } => "prisma.schema",
        }
    }

    pub fn content(&self) -> &str {
        match self {
            SchemaFile::Real { content, .. } | SchemaFile::Virtual { content, .. } => content,
        }
    }
}

impl From<&Path> for SchemaFile<()> {
    fn from(value: &Path) -> Self {
        SchemaFile::from_path(value.to_path_buf())
    }
}
