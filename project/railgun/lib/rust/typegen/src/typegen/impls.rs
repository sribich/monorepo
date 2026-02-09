use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::Generics;
use crate::NamedType;
use crate::Type;
use crate::cache::TypeCache;
use crate::datatype::DataType;
use crate::datatype::NamedDataType;
use crate::datatype::list::ListMeta;
use crate::datatype::primitive::PrimitiveMeta;
use crate::datatype::reference::Reference;
use crate::datatype::tuple::TupleMeta;
use crate::id::TypeId;

// macro_rules! impl_inner {}

/// Primitives
macro_rules! impl_primitives {
    ($($i:ident)+) => {$(
        impl Type for $i {
            fn datatype(_: &mut TypeCache, _: &Generics) -> DataType {
                DataType::Primitive(PrimitiveMeta::$i)
            }
        }
    )+};
}

impl_primitives!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
    bool
    char
    String
);

impl Type for () {
    fn datatype(_cache: &mut TypeCache, _generics: &Generics) -> DataType {
        DataType::Unit
    }
}

macro_rules! impl_containers {
    ($($container:ident)+) => {$(
        impl <T: Type> Type for $container<T> {
            fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
                T::datatype(cache, generics)
            }

            fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
                generics.first().cloned().unwrap_or_else(
                    || T::reference(cache, generics).inner,
                ).into()
            }
        }

        impl<T: NamedType> NamedType for $container<T> {
            const ID: TypeId = T::ID;

            fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType {
                T::named_datatype(cache, generics)
            }
        }
    )+};
}

impl_containers!(Box Rc Arc Cell RefCell Mutex RwLock);

fn get_concrete_generic(generics: &Generics) -> Option<DataType> {
    if let Generics::Concrete(generics) = generics {
        generics.first().cloned()
    } else {
        None
    }
}

impl<T: Type> Type for Option<T> {
    fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
        DataType::Optional(Box::new(
            get_concrete_generic(generics).unwrap_or_else(|| T::datatype(cache, generics)),
        ))
    }

    fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
        DataType::Optional(Box::new(
            generics
                .first()
                .cloned()
                .unwrap_or_else(|| T::reference(cache, generics).inner),
        ))
        .into()
    }
}

macro_rules! impl_iter {
    ($($unique:expr; $ty:path)+) => {$(
        impl<T: Type> Type for $ty {
            fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
                DataType::List(ListMeta {
                    inner_type: Box::new(get_concrete_generic(generics).unwrap_or_else(
                        || T::datatype(cache, generics)
                    )),
                    length: None,
                    unique: $unique,
                })
            }

            fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
                DataType::List(ListMeta {
                    inner_type: Box::new(generics.first().cloned().unwrap_or_else(
                        || T::reference(cache, generics).inner,
                    )),
                    length: None,
                    unique: $unique,
                }).into()
            }
        }
    )+};
}

macro_rules! impl_hashed_iter {
    ($($unique:expr; $ty:path)+) => {$(
        impl<T: Type, S: core::hash::BuildHasher> Type for $ty {
            fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
                DataType::List(ListMeta {
                    inner_type: Box::new(get_concrete_generic(generics).unwrap_or_else(
                        || T::datatype(cache, generics)
                    )),
                    length: None,
                    unique: $unique,
                })
            }

            fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
                DataType::List(ListMeta {
                    inner_type: Box::new(generics.first().cloned().unwrap_or_else(
                        || T::reference(cache, generics).inner,
                    )),
                    length: None,
                    unique: $unique,
                }).into()
            }
        }
    )+};
}

impl_iter!(
    false; Vec<T>
    false; VecDeque<T>
    false; BinaryHeap<T>
    false; LinkedList<T>
    true; BTreeSet<T>
);

impl_hashed_iter!(
    true; HashSet<T, S>
);

macro_rules! impl_as {
    ($($ty:path as $tty:ident)+) => {$(
        impl Type for $ty {
            fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
                <$tty as Type>::datatype(cache, generics)
            }

            fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
                <$tty as Type>::reference(cache, generics)
            }
        }
    )+};
}

const _: () = {
    use std::path::*;

    impl_as!(
        Path as String
        PathBuf as String
    );
};

macro_rules! impl_tuples {
    (
        $($ty:ident),*
    ) => {
        impl<$($ty: Type,)*> Type for ($($ty,)*) {
            fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
                DataType::Tuple(TupleMeta {
                    elements: vec![
                        $(
                            <$ty as Type>::datatype(cache, generics)
                        ),*
                    ],
                })

            }

            fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
                DataType::Tuple(TupleMeta {
                    elements: vec![
                        $(
                            <$ty as Type>::reference(cache, generics).inner
                        ),*
                    ],
                }).into()
            }
        }
    }
}

impl_tuples!(T1);
impl_tuples!(T1, T2);
impl_tuples!(T1, T2, T3);

#[cfg(feature = "chrono")]
const _: () = {
    use chrono::*;

    use crate::cache::TypeCache;

    impl<T: TimeZone> Type for DateTime<T> {
        fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
            <String>::datatype(cache, generics)
        }

        fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
            <String>::reference(cache, generics)
        }
    }
};

#[cfg(feature = "uuid")]
const _: () = {
    use uuid::*;

    impl Type for Uuid {
        fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
            <String>::datatype(cache, generics)
        }

        fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
            <String>::reference(cache, generics)
        }
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use axum::Json;

    impl<T: Type> Type for Json<T> {
        fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
            T::datatype(cache, generics)
        }

        fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
            T::reference(cache, generics)
        }
    }

    impl<T: NamedType> NamedType for Json<T> {
        const ID: TypeId = T::ID;

        fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType {
            T::named_datatype(cache, generics)
        }
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use axum::response::IntoResponse;

    impl<T: NamedType, E: IntoResponse> Type for std::result::Result<T, E> {
        fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
            T::datatype(cache, generics)
        }

        fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
            generics
                .first()
                .cloned()
                .unwrap_or_else(|| T::reference(cache, generics).inner)
                .into()
        }
    }

    impl<T: NamedType, E: IntoResponse> NamedType for std::result::Result<T, E> {
        const ID: TypeId = T::ID;

        fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType {
            T::named_datatype(cache, generics)
        }
    }
};

// #[cfg(feature = "axum")]
// const _: () = {
// use axum::{http::StatusCode, response::IntoResponse};
//
// impl<T: NamedType, E: IntoResponse> Type for std::result::Result<(StatusCode, T), E> {
// fn datatype(cache: &mut TypeCache, generics: &Generics) -> DataType {
// T::datatype(cache, generics)
// }
//
// fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
// T::reference(cache, generics)
// }
// }
//
// impl<T: NamedType, E: NamedType + IntoResponse> NamedType
// for std::result::Result<(StatusCode, T), E>
// {
// const ID: TypeId = T::ID;
//
// fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType {
// let inner_type = T::datatype(cache, generics);
// let inner_type_b = T::named_datatype(cache, generics);
//
// let generics = [];
// let generics = Generics::Concrete(&generics);
//
// T::named_datatype(cache, &generics)
// }
// }
// };
