pub mod model;

use constants::output_fields::*;

use super::*;

pub(crate) fn affected_records_object_type<'a>() -> ObjectType<'a> {
    ObjectType::new(
        Identifier::new_prisma(IdentifierType::AffectedRowsOutput),
        || {
            vec![field_no_arguments(
                AFFECTED_COUNT,
                OutputType::non_list(OutputType::int()),
                None,
            )]
        },
    )
}
