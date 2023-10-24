use alloc::string::{String, ToString};

pub(crate) struct AoCError(String);

impl From<AoCError> for String {
    fn from(value: AoCError) -> Self {
        value.0
    }
}

impl<S: ToString> From<S> for AoCError {
    fn from(value: S) -> Self {
        Self(value.to_string())
    }
}
