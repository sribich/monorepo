use std::convert::identity;
use std::env::current_dir;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

pub const SCHEMA_PATHS: [&str; 1] = ["prisma/schema.prisma"];

#[derive(Clone, Debug)]
pub struct MigrationFiles {
    root: PathBuf,
    files: Vec<MigrationFile>,
}

#[derive(Clone, Debug)]
pub struct MigrationFile {
    base: PathBuf,
    pub subpath: PathBuf,
    name: String,
    content: String,
}

impl MigrationFiles {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        path.try_exists().is_ok_and(identity).then(|| {
            let mut files = MigrationFiles::load_files(&path);

            files.sort_by(|a, b| a.base.to_str().unwrap().cmp(b.base.to_str().unwrap()));

            Self { root: path, files }
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn iter(&self) -> impl Iterator<Item = &MigrationFile> {
        self.files.iter()
    }

    /// Returns the number of migrations found.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    fn load_files(from: &Path) -> Vec<MigrationFile> {
        let files = find_files(from, &|p| p.ends_with("migration.sql"));

        files
            .into_iter()
            .map(|path| {
                let subpath = diff_paths(path.parent().unwrap(), from).unwrap();
                let base = path.parent().unwrap().to_owned();
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();

                let content = std::fs::read_to_string(base.join(&name)).unwrap();

                MigrationFile {
                    base,
                    subpath,
                    name,
                    content,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl MigrationFile {
    pub fn base(&self) -> &Path {
        &self.base
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Clone, Debug)]
pub struct LocatedFiles {
    pub(crate) schema_root: PathBuf,
    pub(crate) schema_files: Vec<PathBuf>,

    pub(crate) migrations: Option<MigrationFiles>,

    pub(crate) sql_root: Option<PathBuf>,
    pub(crate) sql_files: Vec<PathBuf>,
}

impl LocatedFiles {
    pub fn new(cwd: Option<PathBuf>, path: Option<PathBuf>) -> LocatedFiles {
        let schema_root = find_schema_root(cwd, path);
        let schema_files = find_files(
            &schema_root,
            &|p| matches!(p.extension(), Some(extension) if extension == "prisma"),
        );

        let migrations = MigrationFiles::from_path(schema_root.join("migrations"));

        let sql_root_path = schema_root.join("sql");
        let (sql_root, sql_files) = if sql_root_path.try_exists().is_ok_and(identity) {
            (
                Some(sql_root_path.clone()),
                find_files(&sql_root_path, &|p| p.extension().unwrap() == ".sql"),
            )
        } else {
            (None, vec![])
        };

        LocatedFiles {
            schema_root,
            schema_files,
            migrations,
            // migration_root,
            // migration_files,
            sql_root,
            sql_files,
        }
    }

    pub fn root(&self) -> &Path {
        &self.schema_root
    }

    pub fn migrations(&self) -> Option<&MigrationFiles> {
        self.migrations.as_ref()
    }

    pub fn migrations_root(&self) -> Option<&PathBuf> {
        None
        // self.migration_root.as_ref()
    }

    // pub fn migrations(&self) -> Option<Vec<_>> {
    //     self.migration_files
    // }
}

fn find_schema_root(cwd: Option<PathBuf>, path: Option<PathBuf>) -> PathBuf {
    let cwd = cwd.unwrap_or_else(|| current_dir().unwrap());

    if let Some(path) = path {
        let full_path = cwd.join(path);

        if !full_path.try_exists().is_ok_and(identity) {
            panic!("Path does not exist: {:?}", full_path);
        }

        return if full_path.is_file() {
            full_path.parent().unwrap().to_owned()
        } else {
            full_path
        };
    }

    for path in SCHEMA_PATHS {
        let full_path = cwd.join(path);

        if !full_path.try_exists().is_ok_and(identity) {
            continue;
        }

        return if full_path.is_file() {
            full_path.parent().unwrap().to_owned()
        } else {
            full_path
        };
    }

    panic!("Could not find schema root");
}

fn find_files<F>(base: &Path, matcher: &F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    assert!(base.is_dir());

    let mut files = vec![];

    for file in base.read_dir().unwrap() {
        let file = file.unwrap();
        let meta = file.metadata().unwrap();
        let path = file.path();

        if meta.is_file() && matcher(&path) {
            files.push(path);
        } else if meta.is_dir() {
            files.extend(find_files(&path, matcher));
        }
    }

    files
}

/// Construct a relative path from a provided base directory path to the provided path.
///
/// ```rust
/// use std::path::*;
///
/// use pathdiff::diff_paths;
///
/// assert_eq!(diff_paths("/foo/bar", "/foo/bar/baz"), Some("../".into()));
/// assert_eq!(diff_paths("/foo/bar/baz", "/foo/bar"), Some("baz".into()));
/// assert_eq!(
///     diff_paths("/foo/bar/quux", "/foo/bar/baz"),
///     Some("../quux".into())
/// );
/// assert_eq!(
///     diff_paths("/foo/bar/baz", "/foo/bar/quux"),
///     Some("../baz".into())
/// );
/// assert_eq!(diff_paths("/foo/bar", "/foo/bar/quux"), Some("../".into()));
///
/// assert_eq!(diff_paths("/foo/bar", "baz"), Some("/foo/bar".into()));
/// assert_eq!(diff_paths("/foo/bar", "/baz"), Some("../foo/bar".into()));
/// assert_eq!(diff_paths("foo", "bar"), Some("../foo".into()));
///
/// assert_eq!(
///     diff_paths(&"/foo/bar/baz", "/foo/bar".to_string()),
///     Some("baz".into())
/// );
/// assert_eq!(
///     diff_paths(
///         Path::new("/foo/bar/baz"),
///         Path::new("/foo/bar").to_path_buf()
///     ),
///     Some("baz".into())
/// );
/// ```
pub fn diff_paths(path: &Path, base: &Path) -> Option<PathBuf> {
    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();

        let mut comps: Vec<Component> = vec![];

        // ./foo and foo are the same
        if let Some(Component::CurDir) = ita.clone().next() {
            ita.next();
        }

        if let Some(Component::CurDir) = itb.clone().next() {
            itb.next();
        }

        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
