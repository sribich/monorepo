use core::any::TypeId;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DynTypeId(pub TypeId);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ConcreteTypeId(pub TypeId);
