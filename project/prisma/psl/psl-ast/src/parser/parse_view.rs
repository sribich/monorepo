use diagnostics::DatamodelError;
use diagnostics::Diagnostics;
use diagnostics::FileId;

use super::Rule;
use super::helpers::Pair;
use super::helpers::parsing_catch_all;
use super::parse_attribute::parse_attribute;
use super::parse_comments::parse_comment_block;
use super::parse_field::parse_field;
use crate::ast::Attribute;
use crate::ast::{self};

pub(crate) fn parse_view(
    pair: Pair<'_>,
    doc_comment: Option<Pair<'_>>,
    diagnostics: &mut Diagnostics,
    file_id: FileId,
) -> ast::Model {
    let pair_span = pair.as_span();
    let mut name: Option<ast::Identifier> = None;
    let mut fields: Vec<ast::Field> = vec![];
    let mut attributes: Vec<Attribute> = Vec::new();

    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::VIEW_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => (),
            Rule::identifier => name = Some(ast::Identifier::new(current, file_id)),
            Rule::model_contents => {
                let mut pending_field_comment: Option<Pair<'_>> = None;

                for item in current.into_inner() {
                    match item.as_rule() {
                        Rule::block_attribute => {
                            attributes.push(parse_attribute(item, diagnostics, file_id))
                        }
                        Rule::field_declaration => match parse_field(
                            &name.as_ref().unwrap().name,
                            "view",
                            item,
                            pending_field_comment.take(),
                            diagnostics,
                            file_id,
                        ) {
                            Ok(field) => fields.push(field),
                            Err(err) => diagnostics.push_error(err),
                        },
                        Rule::comment_block => pending_field_comment = Some(item),
                        Rule::BLOCK_LEVEL_CATCH_ALL => {
                            diagnostics.push_error(DatamodelError::new_validation_error(
                                "This line is not a valid field or attribute definition.",
                                (file_id, item.as_span()).into(),
                            ))
                        }
                        _ => parsing_catch_all(&item, "view"),
                    }
                }
            }
            _ => parsing_catch_all(&current, "view"),
        }
    }

    match name {
        Some(name) => ast::Model {
            name,
            fields,
            attributes,
            documentation: doc_comment.and_then(parse_comment_block),
            is_view: true,
            span: ast::Span::from((file_id, pair_span)),
        },
        _ => panic!("Encountered impossible model declaration during parsing",),
    }
}
