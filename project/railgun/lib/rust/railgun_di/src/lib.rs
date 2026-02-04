#![feature(macro_metavar_expr_concat, downcast_unchecked, unsize)]

mod component;
mod error;
mod id;
mod injector;
mod injector_builder;
pub mod scope;
mod spec;
mod typecast_builder;

pub use component::{Builder, Component, TypedBuilder};
pub use error::{InjectionError, InjectorError};
pub use injector::Injector;
pub use injector_builder::InjectorBuilder;
pub use railgun_di_derive::*;
pub use scope::Scope;
pub use spec::{OneOf, Spec};

/*
use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    marker::PhantomData,
    sync::Arc,
};

pub use railgun_di_derive::*;

//==============================================================================
//
//==============================================================================

/// Used to constrain F to a specific Fn implementation based on
/// provided generic Args.
pub struct FnDependency<Args, F> {
    f: F,
    marker: PhantomData<fn() -> Args>,
}

pub trait Dependency {
    type Out;

    fn run(&mut self, dependencies: &mut HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self::Out;
}

pub trait IntoDependency<Args> {
    type Dependency: Dependency;

    fn into_dependency(self) -> Self::Dependency;
}

//==============================================================================
//
//==============================================================================
trait DependencyParam {
    type Item<'new>;

    fn receive(resources: &'_ HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self::Item<'_>;
}

pub struct Dep<T: 'static> {
    inner: Arc<T>,
}

impl<T: 'static> Dep<T> {
    pub fn get(&self) -> Arc<T> {
        Arc::clone(&self.inner)
    }
}

impl<T: Send + Sync + 'static> DependencyParam for Dep<T> {
    type Item<'new> = Dep<T>;

    fn receive(resources: &'_ HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self::Item<'_> {
        Dep {
            inner: Arc::clone(
                resources
                    .get(&TypeId::of::<T>())
                    .or_else(|| {
                        panic!("Failed to find {:#?}", type_name::<T>());
                    })
                    .unwrap(),
            )
            .downcast::<T>()
            .unwrap(),
        }
    }
}

// struct DepMut<'a, T: 'static> {
//     inner: &'a mut T,
// }
//
// impl<'a, T: 'static> DepMut<'a, T> {
//     pub fn get(&self) -> &'a mut T {
//         self.inner
//     }
// }
//
// impl<'a, T: 'static> DependencyParam for DepMut<'a, T> {
//     type Item<'new> = DepMut<'new, T>;
//
//     fn receive(resources: &'_ HashMap<TypeId, Box<dyn Any + Send + Sync>>) ->
// Self::Item<'_> {         DepMut {
//             inner: resources
//                 .get_mut(&TypeId::of::<T>())
//                 .unwrap()
//                 .downcast_mut()
//                 .unwrap(),
//         }
//     }
// }
//
// struct DepOwned<T: 'static> {
//     inner: T,
// }
//
// impl<T: 'static> DepOwned<T> {
//     pub fn get(self) -> T {
//         self.inner
//     }
// }
//
// impl<T: 'static> DependencyParam for DepOwned<T> {
//     type Item<'new> = Dep<'new, T>;
//
//     fn receive(resources: &'_ HashMap<TypeId, Box<dyn Any + Send + Sync>>) ->
// Self::Item<'_> {         Dep {
//             inner: *resources
//                 .remove(&TypeId::of::<T>())
//                 .unwrap()
//                 .downcast()
//                 .unwrap(),
//         }
//     }
// }

//==============================================================================
//
//==============================================================================
macro_rules! impl_dependency {
    ($($params:ident),*) => {
        #[allow(clippy::allow_attributes, reason = "we generally want this, but it cant be used in this macro due to how its used")]
        #[allow(unused_variables, reason = "needed for no-param use case")]
        #[allow(non_snake_case, reason = "generic idents are reused as variable names")]
        impl<R, F, $($params : DependencyParam),*> Dependency for FnDependency<($($params ,)*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) -> R +
                FnMut($(<$params as DependencyParam>::Item<'b>),*) -> R
        {
            type Out = R;

            fn run(&mut self, resources: &mut HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self::Out {
                #[allow(clippy::too_many_arguments, reason = "generated")]
                fn call_inner<R, $($params),*>(
                    mut f: impl FnMut($($params),*) -> R,
                    $(${concat(_, $params)}: $params),*
                ) -> R {
                    f($(${concat(_, $params)}),*)
                }

                // // resources.get(&TypeId::of::<$params>()).unwrap().downcast_ref::<$params>().unwrap();
                $(
                    let ${concat(_, $params)} = $params::receive(resources);
                )*

                call_inner(&mut self.f, $(${concat(_, $params)}),*)
            }
        }

        impl<R, F: FnMut($($params),*) -> R, $($params : DependencyParam),*> IntoDependency<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) -> R +
                FnMut($(<$params as DependencyParam>::Item<'b>),*) -> R
        {
            type Dependency = FnDependency<($($params,)*), Self>;

            fn into_dependency(self) -> Self::Dependency {
                FnDependency {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_dependency!();
impl_dependency!(T1);
impl_dependency!(T1, T2);
impl_dependency!(T1, T2, T3);
impl_dependency!(T1, T2, T3, T4);
impl_dependency!(T1, T2, T3, T4, T5);
impl_dependency!(T1, T2, T3, T4, T5, T6);
impl_dependency!(T1, T2, T3, T4, T5, T6, T7);
impl_dependency!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_dependency!(T1, T2, T3, T4, T5, T6, T7, T8, T9);

//==============================================================================
//
//==============================================================================

pub trait Component {}

pub trait Injectable: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> Injectable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct Injector {
    resources: HashMap<TypeId, Arc<dyn Any + Send + Sync + Send + Sync>>,
    impls:
        HashMap<TypeId, Arc<dyn Fn(&mut Injector) -> Arc<dyn Injectable> + Send + Sync + 'static>>,
}

pub struct Implementor<'a, T: ?Sized + Send + Sync + 'static> {
    injector: &'a mut Injector,
    _marker: PhantomData<T>,
}

impl<T: ?Sized + Send + Sync + 'static> Implementor<'_, T> {
    pub fn with<F>(&mut self, provider: F)
    where
        F: (Fn(&mut Injector) -> Arc<dyn Injectable>) + 'static + Send + Sync,
    {
        let type_id = TypeId::of::<T>();

        self.injector.impls.insert(type_id, Arc::new(provider));
    }
}

// struct Concrete {}
pub trait ConcreteBinding {
    type Trait: Send + Sync;

    fn into_inner(self) -> Arc<Self::Trait>;
}

impl Injector {
    pub fn get_trait<T: ?Sized + Send + Sync + 'static>(&self) -> Arc<T> {
        unimplemented!();
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Arc<T> {
        let type_id = TypeId::of::<T>();

        let any = self.resources.get(&type_id);

        assert!(any.is_some(), "Unable to find type {type_id:?}");

        Arc::clone(any.unwrap()).downcast::<T>().unwrap()
    }

    pub fn consume<T: Any + Send + Sync>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();

        if let Some(dep) = self.resources.remove(&type_id) {
            Arc::try_unwrap(Arc::downcast::<T>(dep).unwrap()).ok()
        } else {
            None
        }
    }

    //
    pub fn implement<'a, T>(&'a mut self) -> Implementor<'a, T>
    where
        T: ?Sized + Send + Sync + 'static,
    {
        Implementor::<'a, T> {
            injector: self,
            _marker: PhantomData,
        }
    }

    pub fn provide<T: Any + Send + Sync>(&mut self, provider: T) -> Arc<T> {
        let type_id = TypeId::of::<T>();

        assert!(
            !self.resources.contains_key(&type_id),
            "Type {type_id:?} has already been provided"
        );

        self.resources.insert(type_id, Arc::new(provider));

        self.get::<T>()
    }

    pub fn provide_by<I, R: Send + Sync + 'static, D: Dependency<Out = R> + 'static>(
        &mut self,
        f: impl IntoDependency<I, Dependency = D>,
    ) -> Arc<R> {
        let resource = f.into_dependency().run(&mut self.resources);

        self.provide(resource)
    }

    pub fn provide_with<F, T>(&mut self, provider: F) -> Arc<T>
    where
        T: Any + Send + Sync,
        F: Fn(&mut Injector) -> T,
    {
        let resource = (provider)(self);

        self.provide(resource)
    }

    pub fn run<I, R, D: Dependency<Out = R> + 'static>(
        &mut self,
        f: impl IntoDependency<I, Dependency = D>,
    ) -> D::Out {
        f.into_dependency().run(&mut self.resources)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn foo(int: Dep<i32>) -> i32 {
        *int.inner
    }

    #[test]
    fn basic_test() {
        let mut injector = Injector::default();

        injector.provide(10_i32);

        assert!(injector.run(foo) == 10_i32, "oh no");
    }

    #[test]
    fn cb_test() {
        struct A {
            value: i32,
        }

        struct B {
            value: i32,
        }

        impl B {
            fn new(a: Dep<A>) -> Self {
                Self {
                    value: a.inner.value,
                }
            }
        }

        fn test(b: Dep<B>) -> i32 {
            b.inner.value
        }

        let mut injector = Injector::default();

        injector.provide(A { value: 100 });
        injector.provide_by(B::new);

        assert!(injector.run(test) == 100_i32, "expected 100");
    }

    #[test]
    fn takes_refs() {
        struct A {
            value: i32,
        }

        fn test(a: Dep<A>) -> i32 {
            a.inner.value
        }

        let mut injector = Injector::default();

        injector.provide(A { value: 100 });

        assert!(injector.run(test) == 100_i32, "expected 100");
    }
}
*/
