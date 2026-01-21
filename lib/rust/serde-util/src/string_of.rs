use serde::Deserialize;
use serde::de::Visitor;

/// Parses a well known string value.
///
/// # Example
///
/// ```rust
/// use epub::__for_tests__::StringOf;
/// use serde::Deserialize;
/// use serde_json::from_str;
///
/// #[derive(Deserialize)]
/// pub struct Foo {
///     bar: StringOf<"baz">,
/// }
///
/// fn deserialize() {
///     assert!(from_str::<Foo>(r#"{"bar": "baz"}"#).is_ok());
///     assert!(from_str::<Foo>(r#"{"bar": "bam"}"#).is_err());
/// }
/// ```
#[derive(Debug)]
pub struct StringOf<const T: &'static str>(pub String);

impl<'de, const T: &'static str> Deserialize<'de> for StringOf<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringOfVisitor<const T: &'static str>;

        impl<const T: &'static str> Visitor<'_> for StringOfVisitor<T> {
            type Value = StringOf<T>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str(&format!("Expected the string value '{T}'"))
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(v.to_owned())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == T {
                    Ok(StringOf(v))
                } else {
                    Err(E::custom(format!(
                        "Deserialized value '{v}' does not match the expected value '{T}'"
                    )))
                }
            }
        }

        deserializer.deserialize_str(StringOfVisitor::<T> {})
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json::from_str;

    use super::StringOf;

    #[derive(Deserialize)]
    struct Foo {
        bar: StringOf<"baz">,
    }

    #[test]
    fn inner_value_matches() {
        let deserialized = from_str::<Foo>(r#"{"bar": "baz"}"#).unwrap();

        assert_eq!(deserialized.bar.0, "baz");
    }

    #[test]
    fn mismatch_is_detected() {
        let deserialized = from_str::<Foo>(r#"{"bar": "invalid"}"#);

        assert!(deserialized.is_err());
    }
}
