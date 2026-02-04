pub trait UnwrapInfallible {
    type Type;

    fn unwrap_infallible(self) -> Self::Type;
}

impl<T> UnwrapInfallible for Result<T, core::convert::Infallible> {
    type Type = T;

    fn unwrap_infallible(self) -> Self::Type {
        self.unwrap_or_else(|never| match never {})
    }
}

pub trait IntoValidationError {
    type _Ok;
    type _Err;

    fn validate<P: AsRef<str>, F>(
        self,
        field: P,
        f: F,
    ) -> Result<Result<Self::_Ok, ValidationError>, Self::_Err>
    where
        F: FnOnce(&Self::_Err) -> Option<String>;

    fn validate_many<P: AsRef<str>, F, K>(
        self,
        field: P,
        f: F,
    ) -> Result<Result<Self::_Ok, ValidationError>, Self::_Err>
    where
        F: Fn(&K) -> Option<String>,
        Self::_Err: Iterator<Item = K> + Clone;
}

impl<T, E> IntoValidationError for Result<T, E> {
    type _Err = E;
    type _Ok = T;

    fn validate<P: AsRef<str>, F>(
        self,
        field: P,
        f: F,
    ) -> Result<Result<Self::_Ok, ValidationError>, Self::_Err>
    where
        F: FnOnce(&Self::_Err) -> Option<String>,
    {
        match self {
            Ok(data) => Ok(Ok(data)),
            Err(err) => {
                let validation = f(&err);

                if let Some(error) = validation {
                    return Ok(Err(ValidationError::Single {
                        field: field.as_ref().to_owned(),
                        error,
                    }));
                }

                Err(err)
            },
        }
    }

    fn validate_many<P: AsRef<str>, F, K>(
        self,
        field: P,
        f: F,
    ) -> Result<Result<Self::_Ok, ValidationError>, Self::_Err>
    where
        F: Fn(&K) -> Option<String>,
        E: Iterator<Item = K> + Clone,
    {
        match self {
            Ok(data) => Ok(Ok(data)),
            Err(err) => {
                let orig = err.clone();

                let result = err
                    .map(|err| {
                        let validation = f(&err);

                        if let Some(err) = validation {
                            return Ok(err);
                        }

                        Err(err)
                    })
                    .collect::<Result<Vec<_>, K>>()
                    .map_err(|_| orig)?;

                Ok(Err(ValidationError::new_multiple(
                    field.as_ref().to_owned(),
                    result,
                )))
            },
        }
    }
}

pub enum ValidationError {
    Single { field: String, error: String },
    Multiple(Vec<ValidationError>),
}

impl ValidationError {
    pub fn new_multiple(field: String, errors: Vec<String>) -> Self {
        Self::Multiple(
            errors
                .into_iter()
                .map(|error| ValidationError::Single {
                    field: field.clone(),
                    error,
                })
                .collect::<Vec<_>>(),
        )
    }
}
