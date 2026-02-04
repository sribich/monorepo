use typegen::Typegen;
use typegen_typescript::type_output;

#[cfg(test)]
#[test]
fn container_rename() {
    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename = "test_struct")]
    struct StructTest;

    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename = "test_enum")]
    enum EnumTest {
        A,
    }

    expect_test::expect![["
        type test_struct = Record<string, never>
    "]]
    .assert_eq(&type_output::<StructTest>());

    expect_test::expect![[r#"
        type test_enum = "A"
    "#]]
    .assert_eq(&type_output::<EnumTest>());
}

#[cfg(test)]
#[test]
fn rename_all_enum() {
    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    enum Unit {
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    enum Unnamed {
        TopLeft(u32),
        TopRight(u32),
        BottomLeft(u32),
        BottomRight(u32),
    }

    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    enum Named {
        TopLeft { max_size: u32 },
        TopRight { max_size: u32 },
        BottomLeft { max_size: u32 },
        BottomRight { max_size: u32 },
    }

    expect_test::expect![[r#"
        type Unit = "topLeft" | "topRight" | "bottomLeft" | "bottomRight"
    "#]]
    .assert_eq(&type_output::<Unit>());

    expect_test::expect![[r#"
        type Unnamed = { "topLeft": [ number ] } | { "topRight": [ number ] } | { "bottomLeft": [ number ] } | { "bottomRight": [ number ] }
    "#]].assert_eq(&type_output::<Unnamed>());

    expect_test::expect![[r#"
        type Named = { "topLeft": { "max_size": number } | { "topRight": { "max_size": number } | { "bottomLeft": { "max_size": number } | { "bottomRight": { "max_size": number }
    "#]].assert_eq(&type_output::<Named>());
}

#[cfg(test)]
#[test]
fn rename_all_struct() {
    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    struct Unit;

    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    struct Unnamed(u32, u32, u32, u32);

    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all = "camelCase")]
    struct Named {
        top_left: u32,
        top_right: u32,
        bottom_left: u32,
        bottom_right: u32,
    }

    expect_test::expect![["
        type Unit = Record<string, never>
    "]]
    .assert_eq(&type_output::<Unit>());

    expect_test::expect![["
        [number, number, number, number]
    "]]
    .assert_eq(&type_output::<Unnamed>());

    expect_test::expect![["
        export interface Named {
            topLeft: number
            topRight: number
            bottomLeft: number
            bottomRight: number
        }
    "]]
    .assert_eq(&type_output::<Named>());
}

#[cfg(test)]
#[test]
fn rename_all_fields() {
    #[allow(unused, reason = "Type is exported, not actually unused")]
    #[derive(Typegen)]
    #[serde(rename_all_fields = "camelCase")]
    enum Named {
        TopLeft { max_size: u32 },
        TopRight { max_size: u32 },
        BottomLeft { max_size: u32 },
        BottomRight { max_size: u32 },
    }

    expect_test::expect![[r#"
        type Named = { "TopLeft": { "maxSize": number } | { "TopRight": { "maxSize": number } | { "BottomLeft": { "maxSize": number } | { "BottomRight": { "maxSize": number }
    "#]].assert_eq(&type_output::<Named>());
}
