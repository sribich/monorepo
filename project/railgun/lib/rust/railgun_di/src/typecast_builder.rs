use core::any::Any;
use core::any::TypeId;
use core::marker::PhantomData;
use std::sync::Arc;

use crate::Builder;
use crate::InjectionError;
use crate::Injector;

#[derive(Clone)]
pub struct BindingInner {
    pub caster: Arc<dyn Any + Send + Sync>,
    pub builder: Arc<dyn Builder>,
}

#[derive(Clone)]
pub enum Binding {
    Single(BindingInner),
    Collection(Vec<BindingInner>),
}

impl Binding {
    pub fn new(caster: Arc<dyn Any + Send + Sync>, builder: Arc<dyn Builder>) -> Self {
        Self::Single(BindingInner { caster, builder })
    }

    pub fn new_collection() -> Self {
        Self::Collection(vec![])
    }

    pub fn push(&mut self, caster: Arc<dyn Any + Send + Sync>, builder: Arc<dyn Builder>) {
        if let Binding::Collection(inner) = self {
            inner.push(BindingInner { caster, builder })
        } else {
            // TODO: panic
            panic!("Adding a vec to a non-vec type");
        }
    }
}

pub struct TypeCaster<Into: ?Sized> {
    pub cast: fn(Arc<dyn Any + Send + Sync>) -> Arc<Into>,
}

pub enum BuilderList<'a, TDyn>
where
    TDyn: 'static + ?Sized,
{
    Single(TypecastBuilder<'a, TDyn>),
    Collection(Box<dyn Iterator<Item = TypecastBuilder<'a, TDyn>> + 'a>),
}

/// Takes a dynamic `Builder` and casts the instance to desired interface.
pub struct TypecastBuilder<'a, Iface>
where
    Iface: 'static + ?Sized,
{
    builder: &'a dyn Builder,
    caster: &'a TypeCaster<Iface>,
}

impl<Iface> Builder for TypecastBuilder<'_, Iface>
where
    Iface: 'static + ?Sized,
{
    fn type_id(&self) -> TypeId {
        self.builder.type_id()
    }

    fn type_name(&self) -> &'static str {
        self.builder.type_name()
    }

    fn as_any(&self, injector: &Injector) -> Result<Arc<dyn Any + Send + Sync>, InjectionError> {
        self.builder.as_any(injector)
    }

    /*
    fn interfaces(&self, clb: &mut dyn FnMut(&InterfaceDesc) -> bool) {
        self.builder.interfaces(clb);
    }

    fn metadata<'b, 'c>(&'b self, clb: &'c mut dyn FnMut(&'b dyn Any) -> bool) {
        self.builder.metadata(clb)
    }

    fn check(&self, cat: &Catalog) -> Result<(), ValidationError> {
        self.builder.check(cat)
    }
    */
}

impl<'a, TDyn> TypecastBuilder<'a, TDyn>
where
    TDyn: 'static + ?Sized,
{
    pub(crate) fn new(builder: &'a dyn Builder, caster: &'a TypeCaster<TDyn>) -> Self {
        Self { builder, caster }
    }

    pub fn get(&self, injector: &Injector) -> Result<Arc<TDyn>, InjectionError> {
        let instance = self.builder.as_any(injector)?;

        Ok((self.caster.cast)(instance))
    }
}

/*
pub(crate) struct TypecastBuilderIterator<'a, Iface: 'static + ?Sized> {
    bindings: Option<&'a Vec<Binding>>,
    pos: usize,
    _dummy: PhantomData<Iface>,
}

impl<'a, Iface: 'static + ?Sized> TypecastBuilderIterator<'a, Iface> {
    pub(crate) fn new(bindings: Option<&'a Vec<Binding>>) -> Self {
        Self {
            bindings,
            pos: 0,
            _dummy: PhantomData,
        }
    }
}

impl<'a, Iface: 'static + ?Sized> Iterator for TypecastBuilderIterator<'a, Iface> {
    type Item = TypecastBuilder<'a, Iface>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(bindings) = self.bindings
            && self.pos < bindings.len()
        {
            let b = &bindings[self.pos];
            self.pos += 1;

            // SAFETY: the TypeID key of the `bindings` map is guaranteed to match the
            // `Iface` type
            let caster: &TypeCaster<Iface> = b.caster.downcast_ref().unwrap();
            return Some(TypecastBuilder::new(b.builder.as_ref(), caster));
        }
        None
    }
}
*/

pub struct TypecastBuilderIterator<'a, TDyn>
where
    TDyn: 'static + ?Sized,
{
    bindings: Option<&'a Vec<BindingInner>>,
    pos: usize,
    _marker: PhantomData<TDyn>,
}

impl<'a, TDyn> TypecastBuilderIterator<'a, TDyn>
where
    TDyn: 'static + ?Sized,
{
    pub fn new(bindings: Option<&'a Vec<BindingInner>>) -> Self {
        Self {
            bindings,
            pos: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, TDyn> Iterator for TypecastBuilderIterator<'a, TDyn>
where
    TDyn: 'static + ?Sized,
{
    type Item = TypecastBuilder<'a, TDyn>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(bindings) = self.bindings
            && self.pos < bindings.len()
        {
            let binding = &bindings.get(self.pos);
            self.pos += 1;

            if let Some(binding) = binding {
                let caster: &TypeCaster<TDyn> = binding.caster.downcast_ref().unwrap();

                return Some(TypecastBuilder::new(binding.builder.as_ref(), caster));
            }
        }

        None
    }
}
