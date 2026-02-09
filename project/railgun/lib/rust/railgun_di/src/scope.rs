use core::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

pub trait Scope {
    fn get(&self) -> Option<Arc<dyn Any + Send + Sync>>;
    fn set(&self, value: Arc<dyn Any + Send + Sync>);
}

#[derive(Default)]
pub struct Static {
    instance: RwLock<Option<Arc<dyn Any + Send + Sync>>>,
}

impl Static {
    pub fn new() -> Self {
        Static::default()
    }
}

impl Scope for Static {
    fn get(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        self.instance.read().unwrap().clone()
    }

    fn set(&self, value: Arc<dyn Any + Send + Sync>) {
        let mut lock = self.instance.write().unwrap();

        *lock = Some(value);
    }
}

#[derive(Default)]
pub struct Transient;

impl Transient {
    pub fn new() -> Self {
        Transient
    }
}

impl Scope for Transient {
    fn get(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    fn set(&self, _value: Arc<dyn Any + Send + Sync>) {
        // noop for transient lifetimes
    }
}
