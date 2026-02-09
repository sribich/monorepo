pub use railgun_api_derive::*;

pub mod json {
    use std::borrow::Cow;
    use std::convert::Infallible;

    use axum::Json;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use serde::Serialize;
    use typegen::Generics;
    use typegen::NamedType;
    use typegen::Type;
    use typegen::Typegen;
    use typegen::cache::TypeCache;
    use typegen::datatype::DataType;
    use typegen::datatype::NamedDataType;
    use typegen::datatype::field::field;
    use typegen::datatype::reference::Reference;
    use typegen::datatype::reference::ReferenceType;
    use typegen::datatype::r#struct::StructFields;
    use typegen::datatype::r#struct::StructType;
    use typegen::id::TypeId;

    /// # Example
    ///
    /// ```rust
    /// #[derive(ApiError, Serialize, Typegen)]
    /// #[api(format = "json")]
    /// #[serde(untagged, rename = "CreateCardError")]
    /// pub enum ApiError {
    ///     #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    ///     Unknown(ApiErrorKind<Option<()>>),
    /// }
    /// ```
    pub type ApiResult<T, E> = Result<ApiResponse<T, E>, E>;

    pub struct ApiResponse<T, E>((StatusCode, Json<ApiBody<T, E>>))
    where
        T: Type + Serialize,
        E: Type + Serialize;

    impl<T, E> ApiResponse<T, E>
    where
        T: Type + Serialize,
        E: Type + Serialize,
    {
        pub fn success(code: StatusCode, data: T) -> Result<Self, E> {
            Ok(Self((code, Json(ApiBody::Success(data)))))
        }

        pub fn failure(code: StatusCode, data: E) -> Result<Self, Infallible> {
            Ok(Self((code, Json(ApiBody::Error(data)))))
        }
    }

    impl<T, E> IntoResponse for ApiResponse<T, E>
    where
        T: Type + Serialize,
        E: Type + Serialize,
    {
        fn into_response(self) -> axum::response::Response {
            self.0.into_response()
        }
    }

    impl<T, E> Type for ApiResponse<T, E>
    where
        T: NamedType + Serialize,
        E: NamedType + Serialize,
    {
        fn datatype(cache: &mut TypeCache, generics: &Generics) -> typegen::datatype::DataType {
            let delegate_dt = T::named_datatype(&mut TypeCache::default(), &Generics::Impl);
            let name = Cow::Owned(format!("{}Aggregate", delegate_dt.name()));

            let f = {
                let t = T::reference(cache, &[]).inner;
                let e = E::reference(cache, &[]).inner;

                ApiBody::<T, E>::reference(cache, &[t, e]).inner
            };

            DataType::Struct(StructType::new(
                name,
                <Self as NamedType>::ID,
                StructFields::new_unnamed(vec![field(Some(f), String::new(), None, false)]),
                vec![],
            ))
        }

        fn reference(cache: &mut TypeCache, generics: &[DataType]) -> Reference {
            let delegate_dt = T::named_datatype(&mut TypeCache::default(), &Generics::Impl);
            let name = Cow::Owned(format!("{}Aggregate", delegate_dt.name()));

            Reference::new_named::<Self>(
                cache,
                ReferenceType::new(name, vec![], <Self as NamedType>::ID),
            )
        }
    }

    impl<T, E> NamedType for ApiResponse<T, E>
    where
        T: NamedType + Serialize,
        E: NamedType + Serialize,
    {
        const ID: TypeId = ApiBody::<T, E>::ID.delegate(T::ID);

        fn named_datatype(cache: &mut TypeCache, generics: &Generics) -> NamedDataType {
            let generics = [
                T::datatype(cache, &Generics::Impl),
                E::datatype(cache, &Generics::Impl),
            ];
            let generics = Generics::Concrete(&generics);

            let original_dt = T::named_datatype(cache, &generics);

            NamedDataType::new(
                Cow::Owned(format!("{}Aggregate", original_dt.name())),
                Self::ID,
                <Self as Type>::datatype(cache, &generics),
                String::new(),
                None,
            )
        }
    }

    #[derive(Serialize, Typegen)]
    #[serde(untagged)]
    pub enum ApiBody<T, E>
    where
        T: Serialize + Type,
        E: Serialize + Type,
    {
        Success(T),
        Error(E),
    }

    #[derive(Serialize, Typegen)]
    pub struct ApiErrorKind<E = ()>
    where
        E: Serialize + Type,
    {
        #[serde(skip)]
        pub source: Box<dyn core::error::Error>,
        pub code: String,
        #[serde(flatten)]
        pub error: ApiErrorData<E>,
    }

    impl<E> ApiErrorKind<E>
    where
        E: Serialize + Type + Default,
    {
        pub fn error<S: AsRef<str>>(source: impl std::error::Error + 'static, error: S) -> Self {
            tracing::error!("{}", source);

            Self {
                source: Box::new(source),
                code: String::new(),
                error: ApiErrorData::error(error),
            }
        }
    }

    #[derive(Serialize, Typegen)]
    pub struct ApiErrorData<E>
    where
        E: Serialize + Type,
    {
        pub data: E,
        pub error: String,
    }

    impl<E> ApiErrorData<E>
    where
        E: Serialize + Type + Default,
    {
        pub fn error<S: AsRef<str>>(error: S) -> Self {
            Self {
                data: Default::default(),
                error: error.as_ref().into(),
            }
        }
    }

    #[derive(Serialize, Typegen)]
    pub struct ApiErrorInternal<E>
    where
        E: Serialize + Type,
    {
        pub code: String,
        #[serde(flatten)]
        pub kind: ApiErrorKind<E>,
    }

    #[derive(Debug, Serialize, Typegen, Default)]
    pub struct UnknownApiError;

    impl std::fmt::Display for UnknownApiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Unknown API error")
        }
    }

    impl std::error::Error for UnknownApiError {}
}
