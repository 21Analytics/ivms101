use crate::messages::ValidationError;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "Vec<T>", try_from = "Vec<T>")]
pub(crate) struct NonEmptyVec<T: Clone> {
    inner: Vec<T>,
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T>
where
    T: Clone,
{
    type Error = ValidationError;
    fn try_from(from: Vec<T>) -> Result<Self, ValidationError> {
        match from.len() {
            0 => Err("Vector must not be empty".into()),
            _ => Ok(Self { inner: from }),
        }
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    fn from(nev: NonEmptyVec<T>) -> Self {
        nev.inner
    }
}

impl<T: Clone> From<T> for NonEmptyVec<T> {
    fn from(element: T) -> Self {
        Self {
            inner: vec![element],
        }
    }
}

impl<T: Clone> NonEmptyVec<T> {
    pub(crate) fn first(&self) -> &T {
        self.inner.first().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_non_empty_vec() {
        serde_test::assert_tokens(
            &super::NonEmptyVec::from(1_u8),
            &[
                serde_test::Token::Seq { len: Some(1) },
                serde_test::Token::U8(1),
                serde_test::Token::SeqEnd,
            ],
        );

        serde_test::assert_de_tokens_error::<super::NonEmptyVec<u8>>(
            &[
                serde_test::Token::Seq { len: None },
                serde_test::Token::SeqEnd,
            ],
            "Validation error: Vector must not be empty",
        );
    }
}
