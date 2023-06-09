use crate::types::non_empty_vec::NonEmptyVec;

/// `OneToN` is a helper enum to accept a singleton or non-empty list-enumerated
/// field during deserialization.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OneToN<T: Clone> {
    One(T),
    N(NonEmptyVec<T>),
}

impl<T: Clone> OneToN<T> {
    pub fn first(&self) -> &T {
        match self {
            OneToN::One(t) => t,
            OneToN::N(nev_t) => nev_t.first(),
        }
    }
}

impl<T: Clone> From<T> for OneToN<T> {
    fn from(from: T) -> Self {
        OneToN::One(from)
    }
}

impl<T: Clone> IntoIterator for OneToN<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneToN::One(t) => vec![t].into_iter(),
            OneToN::N(nev) => {
                let v: Vec<T> = nev.into();
                v.into_iter()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_serialization() {
        assert_tokens(&OneToN::<u8>::One(1), &[Token::U8(1)]);
        assert_tokens(
            &OneToN::<u8>::N(1.into()),
            &[Token::Seq { len: Some(1) }, Token::U8(1), Token::SeqEnd],
        );
        assert_tokens(
            &OneToN::<u8>::N(vec![1, 2].try_into().unwrap()),
            &[
                Token::Seq { len: Some(2) },
                Token::U8(1),
                Token::U8(2),
                Token::SeqEnd,
            ],
        );
        serde_test::assert_de_tokens_error::<OneToN<u8>>(
            &[Token::Seq { len: None }, Token::SeqEnd],
            "data did not match any variant of untagged enum OneToN",
        );
    }
}
