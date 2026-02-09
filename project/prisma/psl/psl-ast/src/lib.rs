//! The Prisma Schema AST.
//!
//! This crate handles the parsing and reformatting of the Prisma AST.
#![deny(rust_2018_idioms, unsafe_code)]
#![allow(clippy::derive_partial_eq_without_eq)]
mod parser;
mod reformat;
mod renderer;
mod source_file;

use diagnostics::FileId;
use psl_schema::DefaultRefiner;
use psl_schema::Schema;
use psl_schema::SchemaFile;
use psl_schema::SchemaRefiner;
pub use source_file::SourceFile;

pub use self::parser::parse_schema;
pub use self::reformat::reformat;

/// The AST data structure. It aims to faithfully represent the syntax of a Prisma Schema, with
/// source span information.
pub mod ast;

#[derive(Clone, Debug)]
pub struct ParsedFile {
    ast: ast::SchemaAst,
}

impl ParsedFile {
    pub fn ast(&self) -> &ast::SchemaAst {
        &self.ast
    }
}

///
pub struct Parsed;

impl SchemaRefiner for Parsed {
    type FileContext = ParsedFile;
    type From = DefaultRefiner;
    type SchemaContext = ();

    fn refine_context(&self, from: &Schema<Self::From>) -> Self::SchemaContext {
        ()
    }

    fn refine_file(
        &self,
        from: &Schema<Self::From>,
        context: &mut Self::SchemaContext,
        file: &SchemaFile<<Self::From as SchemaRefiner>::FileContext>,
    ) -> Self::FileContext {
        /*
        let files = schema.schema_files().clone();

        let ast = files
            .iter()
            .map(|it| parse_schema(it.content(), &mut *schema.diagnostics().borrow_mut(), FileId(0)))
            .collect::<Vec<_>>();
         */
        let ast = parse_schema(
            file.content(),
            &mut *from.diagnostics().borrow_mut(),
            FileId(0),
        );

        ParsedFile { ast }
    }
}

pub trait SchemaParser {
    fn parse(self) -> Schema<Parsed>;
}

impl SchemaParser for Schema {
    fn parse(self) -> Schema<Parsed> {
        self.refine(Parsed {})
    }
}

/// Transform the input string into a valid (quoted and escaped) PSL string literal.
///
/// PSL string literals have the exact same grammar as [JSON string
/// literals](https://datatracker.ietf.org/doc/html/rfc8259#section-7).
///
/// ```
/// # use psl_ast::string_literal;
/// let input = r#"oh
/// hi"#;
/// assert_eq!(r#""oh\nhi""#, &string_literal(input).to_string());
/// ```
pub fn string_literal(s: &str) -> impl std::fmt::Display + '_ {
    struct StringLiteral<'a>(&'a str);

    impl std::fmt::Display for StringLiteral<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("\"")?;
            for c in self.0.char_indices() {
                match c {
                    (_, '\t') => f.write_str("\\t")?,
                    (_, '\n') => f.write_str("\\n")?,
                    (_, '"') => f.write_str("\\\"")?,
                    (_, '\r') => f.write_str("\\r")?,
                    (_, '\\') => f.write_str("\\\\")?,
                    // Control characters
                    (_, c) if c.is_ascii_control() => {
                        let mut b = [0];
                        c.encode_utf8(&mut b);
                        f.write_fmt(format_args!("\\u{:04x}", b[0]))?;
                    }
                    (start, other) => f.write_str(&self.0[start..(start + other.len_utf8())])?,
                }
            }
            f.write_str("\"")
        }
    }

    StringLiteral(s)
}
