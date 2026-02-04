use std::{collections::HashMap, fs::File, io::Write, path::Path, str::FromStr};

use railgun_core::bootstrap::BootstrapResult;
use railgun_error::ResultExt;
pub use railgun_settings_derive::settings;
use serde::{Serialize, de::DeserializeOwned};
use toml_edit::visit_mut::VisitMut;

mod impls;
pub mod net;

pub trait Settings: Default + Clone + Serialize + DeserializeOwned + 'static {
    fn add_docs(
        &self,
        _parent_key: &[String],
        _docs: &mut HashMap<Vec<String>, &'static [&'static str]>,
    ) {
    }
}

/// Write serialized settings to a file.
pub fn to_file<T: Settings>(settings: &T, path: impl AsRef<Path>) -> BootstrapResult<()> {
    let mut file = File::create(path).boxed()?;
    let data = to_str(settings)?;

    file.write_all(data.as_bytes()).boxed()?;

    Ok(())
}

/// Parse settings from a TOML file.
pub fn from_file<T: Settings>(path: impl AsRef<Path>) -> BootstrapResult<T> {
    let data = std::fs::read_to_string(path).boxed()?;

    from_str(data)
}

pub fn to_str<T: Settings>(settings: &T) -> BootstrapResult<String> {
    let mut doc_comments = HashMap::default();

    settings.add_docs(&[], &mut doc_comments);

    let mut document = toml_edit::ser::to_document(settings).boxed()?;
    let mut visitor = CommentVisitor {
        comments: doc_comments,
        path: vec![],
    };

    visitor.visit_document_mut(&mut document);

    // let value =
    // toml_edit::ser::to_string_pretty(&from_str(document.to_string())?)?;
    let value = document.to_string();

    /*
    let result = r#"
        name = "foo"
        # This is a comment
        [foo]
        # Another comment
        bar = ""
    "#;

    println!("{:#?}", Document::from_str(result));

    println!("{:#?}", result);
    */

    Ok(value)
}

/// Parse settings from a TOML string.
pub fn from_str<T: Settings>(data: impl AsRef<str>) -> BootstrapResult<T> {
    let de = toml_edit::de::Deserializer::from_str(data.as_ref()).boxed()?;

    Ok(serde_path_to_error::deserialize(de).boxed()?)
}

struct CommentVisitor {
    comments: HashMap<Vec<String>, &'static [&'static str]>,
    path: Vec<String>,
}

impl VisitMut for CommentVisitor {
    // TODO: Decor the document itself
    fn visit_document_mut(&mut self, node: &mut toml_edit::DocumentMut) {
        toml_edit::visit_mut::visit_document_mut(self, node);
    }

    fn visit_item_mut(&mut self, node: &mut toml_edit::Item) {
        try_convert_to_table(node);

        toml_edit::visit_mut::visit_item_mut(self, node);
    }

    fn visit_value_mut(&mut self, node: &mut toml_edit::Value) {
        node.decor_mut().clear();

        toml_edit::visit_mut::visit_value_mut(self, node);
    }

    fn visit_table_mut(&mut self, node: &mut toml_edit::Table) {
        node.decor_mut().clear();

        // Empty tables could be semantically meaningful, so make sure they are not
        // implicit
        if !node.is_empty() {
            node.set_implicit(true);
        }

        toml_edit::visit_mut::visit_table_mut(self, node);
    }

    fn visit_table_like_kv_mut(
        &mut self,
        mut key: toml_edit::KeyMut<'_>,
        node: &mut toml_edit::Item,
    ) {
        self.path.push(key.to_string());

        let comments = self.comments.get(&self.path).map(|vec| {
            vec.iter()
                .map(|line| format!("#{line}"))
                .collect::<Vec<_>>()
                .join("\n")
        });

        if let Some(mut comment) = comments {
            comment.push('\n');

            match node {
                toml_edit::Item::ArrayOfTables(_) => {
                    todo!();
                },
                toml_edit::Item::None => {
                    todo!();
                },
                toml_edit::Item::Table(_) => {
                    todo!();
                },
                toml_edit::Item::Value(item) => {
                    if let toml_edit::Value::InlineTable(table) = item {
                        table.decor_mut().set_prefix(comment);
                    } else {
                        key.leaf_decor_mut().set_prefix(comment);
                    }
                },
            }
        }

        toml_edit::visit_mut::visit_table_like_kv_mut(self, key, node);

        self.path.pop();
    }

    fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
        toml_edit::visit_mut::visit_array_mut(self, node);

        if (0..=1).contains(&node.len()) {
            node.set_trailing("");
            node.set_trailing_comma(false);
        } else {
            for item in node.iter_mut() {
                item.decor_mut().set_prefix("\n    ");
            }
            node.set_trailing("\n");
            node.set_trailing_comma(true);
        }
    }
}

/// Attempts to convert an item into table form.
///
/// When reading configurations, table form is slightly easier to digest.
fn try_convert_to_table(node: &mut toml_edit::Item) {
    let other = std::mem::take(node);
    let other = match other.into_table().map(toml_edit::Item::Table) {
        Err(i) | Ok(i) => i,
    };
    let other = match other
        .into_array_of_tables()
        .map(toml_edit::Item::ArrayOfTables)
    {
        Ok(i) | Err(i) => i,
    };

    *node = other;
}
