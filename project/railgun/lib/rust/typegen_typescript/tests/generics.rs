/*
use typegen::{
    cache::TypeCache,
    export::{lang::typescript::Typescript, TypeExporter},
    NamedType, Typegen,
};

#[test]
fn foobar() {
    #[allow(unused)]
    #[derive(Typegen)]
    struct ItemWrapper<T> {
        item: T,
    }

    let mut cache = TypeCache::default();
    let dt = ItemWrapper::<u32>::named_datatype(&mut cache);

    let output = Typescript::export((), dt, &cache).unwrap();

    expect_test::expect![[r#"
        export interface ItemWrapper<T> {
            item: T
        }
    "#]]
    .assert_eq(&output);
}

#[test]
fn type_with_nested_generic_field() {
    #[allow(unused)]
    #[derive(Typegen)]
    struct Item<T> {
        inner: T,
    }

    #[allow(unused)]
    #[derive(Typegen)]
    struct Other {}

    #[allow(unused)]
    #[derive(Typegen)]
    struct ItemWrapper {
        item: Item<Other>,
    }

    let mut cache = TypeCache::default();
    let dt = ItemWrapper::named_datatype(&mut cache);

    let output = Typescript::export((), dt, &cache).unwrap();

    expect_test::expect![[r#"
        type Other = Record<string, never>

        export interface Item<T> {
            inner: T
        }

        export interface ItemWrapper {
            item: Item<Other>
        }
    "#]]
    .assert_eq(&output);
}

#[test]
fn recursive_type() {
    #[allow(unused)]
    #[derive(Typegen)]
    struct A {
        inner: Box<B>,
    }

    #[allow(unused)]
    #[derive(Typegen)]
    struct B {
        inner: Box<A>,
    }

    let mut cache = TypeCache::default();
    let dt = A::named_datatype(&mut cache);

    let output = Typescript::export((), dt, &cache).unwrap();

    expect_test::expect![[r#"
        export interface B {
            inner: A
        }

        export interface A {
            inner: B
        }
    "#]]
    .assert_eq(&output);
}

fn leak() {
    let data = vec![0; 1024];
    std::mem::forget(data);
}

#[test]
fn test_leak() {
    leak();
}
*/
