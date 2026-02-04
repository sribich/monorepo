use std::fmt::Write as _;

use typegen::internal::Deprecation;

#[derive(Default)]
pub(super) struct Jsdoc {
    description: Option<String>,
    deprecation: Option<Deprecation>,
}

impl Jsdoc {
    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.description = Some(description.into());
    }

    pub fn set_deprecated(&mut self, deprecation: Deprecation) {
        self.deprecation = Some(deprecation);
    }

    pub fn build(&self) -> String {
        let mut jsdoc = String::new();

        if let Some(description) = &self.description {
            description
                .split('\n')
                .for_each(|line| Self::append_maybe_create_string(&mut jsdoc, line));
        }

        if !jsdoc.is_empty() {
            jsdoc.push_str(" */\n");
        }

        jsdoc
    }

    fn append_maybe_create_string(existing: &mut String, content: &str) {
        if existing.is_empty() {
            existing.push_str("/*\n");
        }

        let maybe_space = if content.starts_with(' ') { "" } else { " " };

        writeln!(existing, " *{maybe_space}{content}").unwrap();
    }
}
