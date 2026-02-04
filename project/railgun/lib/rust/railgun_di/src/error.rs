use core::any::TypeId;

use railgun_error::{Error, Location};

#[derive(Error)]
pub enum InjectionError {
    #[error(display("Unregistered type: {type_name}"))]
    Unregistered {
        type_id: TypeId,
        type_name: &'static str,
        location: Location,
    },
    #[error(display("Ambiguous type: {type_name}"))]
    Ambiguous {
        type_id: TypeId,
        type_name: &'static str,
        location: Location,
    },
    #[error(display(
        "injector::get was called on '{type_name}' which returned a collection. Did you mean to call get_vec?"
    ))]
    CalledGetOnCollection {
        type_id: TypeId,
        type_name: &'static str,
        location: Location,
    },
}

#[derive(Error)]
pub enum InjectorError {
    #[error(display("Builder for type {type_name} is already registered"))]
    AlreadyRegistered {
        type_id: TypeId,
        type_name: &'static str,
        location: Location,
    },
    #[error(display("Attmpted to bind to target {type_name} that is not yet registered"))]
    NoBindTarget {
        type_id: TypeId,
        type_name: &'static str,
        location: Location,
    },
}
