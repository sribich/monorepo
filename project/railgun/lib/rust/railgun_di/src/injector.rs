use core::any::{TypeId, type_name};
use std::{collections::HashMap, sync::Arc};

use crate::{
    Builder, InjectionError, OneOf,
    id::{ConcreteTypeId, DynTypeId},
    injector_builder::InjectorBuilder,
    spec::AllOf,
    typecast_builder::{
        Binding, BuilderList, TypeCaster, TypecastBuilder, TypecastBuilderIterator,
    },
};

pub struct Injector(pub(crate) Arc<InjectorInner>);

impl Injector {
    pub fn builder() -> InjectorBuilder {
        InjectorBuilder::new()
    }

    pub(crate) fn new(
        builders: HashMap<ConcreteTypeId, Arc<dyn Builder>>,
        bindings: HashMap<DynTypeId, Binding>,
    ) -> Self {
        Self(Arc::new(InjectorInner { builders, bindings }))
    }

    #[track_caller]
    pub fn get<T>(&self) -> Result<Arc<T>, InjectionError>
    where
        T: 'static + ?Sized + Send + Sync,
    {
        self.spec::<OneOf<T>>()
    }

    #[track_caller]
    pub fn get_vec<T>(&self) -> Result<Vec<Arc<T>>, InjectionError>
    where
        T: 'static + ?Sized + Send + Sync,
    {
        self.spec::<AllOf<T>>()
    }

    #[track_caller]
    pub fn spec<Spec>(&self) -> Result<Spec::Out, InjectionError>
    where
        Spec: crate::Spec,
    {
        Spec::get(self)
    }
}

pub struct InjectorInner {
    #[expect(unused, reason = "Will be used soon")]
    pub(crate) builders: HashMap<ConcreteTypeId, Arc<dyn Builder>>,
    pub(crate) bindings: HashMap<DynTypeId, Binding>,
}

impl InjectorInner {
    #[track_caller]
    pub fn builders_for<TDyn>(&'_ self) -> Result<BuilderList<'_, TDyn>, InjectionError>
    where
        TDyn: 'static + ?Sized,
    {
        let dyn_type = DynTypeId(TypeId::of::<TDyn>());
        let bindings = self.bindings.get(&dyn_type);

        Ok(match bindings {
            Some(a) => match a {
                Binding::Single(binding_inner) => {
                    let caster: &TypeCaster<TDyn> = binding_inner.caster.downcast_ref().unwrap();

                    BuilderList::Single(TypecastBuilder::new(
                        binding_inner.builder.as_ref(),
                        caster,
                    ))
                },
                Binding::Collection(binding_inners) => {
                    // TODO: This doesn't need some()
                    BuilderList::Collection(Box::new(TypecastBuilderIterator::new(Some(
                        binding_inners,
                    ))))
                },
            },
            None => {
                return crate::error::UnregisteredContext {
                    type_id: dyn_type.0,
                    type_name: type_name::<TDyn>(),
                }
                .fail();
            },
        })
    }
}
