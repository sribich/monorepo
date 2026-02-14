#![feature(macro_metavar_expr_concat)]
#![feature(result_option_map_or_default)]
#![feature(associated_type_defaults)]
#![feature(more_qualified_paths)]
pub use features::shared::infra::http::AppState;

pub mod system;

pub mod startup;
