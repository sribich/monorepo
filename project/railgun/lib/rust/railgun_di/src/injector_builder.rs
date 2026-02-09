use core::any::TypeId;
use core::any::type_name;
use core::marker::Unsize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::Builder;
use crate::Component;
use crate::Injector;
use crate::InjectorError;
use crate::TypedBuilder;
use crate::error::AlreadyRegisteredContext;
use crate::id::ConcreteTypeId;
use crate::id::DynTypeId;
use crate::typecast_builder::Binding;
use crate::typecast_builder::TypeCaster;

#[derive(Clone, Default)]
pub struct InjectorBuilder {
    builders: HashMap<ConcreteTypeId, Arc<dyn Builder>>,
    bindings: HashMap<DynTypeId, Binding>,
}

impl InjectorBuilder {
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
            bindings: HashMap::new(),
        }
    }

    #[track_caller]
    pub fn add<C>(&mut self) -> Result<&mut Self, InjectorError>
    where
        C: 'static + Component,
    {
        self.add_builder(C::builder())
    }

    #[track_caller]
    pub fn add_builder<TBuilder, TConcrete>(
        &mut self,
        builder: TBuilder,
    ) -> Result<&mut Self, InjectorError>
    where
        TBuilder: 'static + TypedBuilder<TConcrete>,
        TConcrete: 'static + Send + Sync,
    {
        let concrete_id = ConcreteTypeId(TypeId::of::<TConcrete>());

        if self.builders.contains_key(&concrete_id) {
            return AlreadyRegisteredContext {
                type_id: concrete_id.0,
                type_name: type_name::<TConcrete>(),
            }
            .fail();
        }

        let builder = Arc::new(builder);
        self.builders
            .insert(concrete_id, Arc::clone(&builder) as Arc<dyn Builder>);

        self.bindings.insert(
            DynTypeId(TypeId::of::<TConcrete>()),
            Binding::new(
                Arc::new(TypeCaster::<TConcrete> {
                    cast: |v| v.downcast().unwrap(),
                }),
                Arc::clone(&builder) as Arc<dyn Builder>,
            ),
        );

        (*builder).bind_interfaces(self);

        Ok(self)
    }

    #[track_caller]
    pub fn add_value<TConcrete>(&mut self, value: TConcrete) -> Result<&mut Self, InjectorError>
    where
        TConcrete: 'static + Send + Sync,
    {
        self.add_builder(Arc::new(value))
    }

    pub fn bind<TDyn, TConcrete>(&mut self) -> Result<&mut Self, InjectorError>
    where
        TDyn: 'static + ?Sized,
        TConcrete: 'static + Send + Sync + Unsize<TDyn>,
    {
        let concrete_id = ConcreteTypeId(TypeId::of::<TConcrete>());
        let dyn_id = DynTypeId(TypeId::of::<TDyn>());

        let builder = self.builders.get(&concrete_id);

        if builder.is_none() {
            return crate::error::NoBindTargetContext {
                type_id: concrete_id.0,
                type_name: type_name::<TConcrete>(),
            }
            .fail();
        }

        self.bindings.insert(
            dyn_id,
            Binding::new(
                Arc::new(TypeCaster::<TDyn> {
                    cast: |v| {
                        let s: Arc<TConcrete> = v.downcast().unwrap();
                        let t: Arc<TDyn> = s;
                        t
                    },
                }),
                Arc::clone(builder.unwrap()),
            ),
        );

        Ok(self)
    }

    pub fn bind_vec<TDyn, TConcrete>(&mut self) -> Result<&mut Self, InjectorError>
    where
        TDyn: 'static + ?Sized,
        TConcrete: 'static + Send + Sync + Unsize<TDyn>,
    {
        let concrete_id = ConcreteTypeId(TypeId::of::<TConcrete>());
        let dyn_id = DynTypeId(TypeId::of::<TDyn>());

        let builder = self.builders.get(&concrete_id);

        if builder.is_none() {
            return crate::error::NoBindTargetContext {
                type_id: concrete_id.0,
                type_name: type_name::<TConcrete>(),
            }
            .fail();
        }

        if !self.bindings.contains_key(&dyn_id) {
            self.bindings
                .insert(dyn_id.clone(), Binding::new_collection());
        }

        let bindings = self.bindings.get_mut(&dyn_id).unwrap();

        bindings.push(
            Arc::new(TypeCaster::<TDyn> {
                cast: |v| {
                    let s: Arc<TConcrete> = v.downcast().unwrap();
                    let t: Arc<TDyn> = s;
                    t
                },
            }),
            Arc::clone(builder.unwrap()),
        );

        Ok(self)
    }

    pub fn build(&mut self) -> Injector {
        Injector::new(
            core::mem::take(&mut self.builders),
            core::mem::take(&mut self.bindings),
        )
    }
}
