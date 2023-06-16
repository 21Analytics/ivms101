/// `ZeroToN` is a helper enum to accept an absent, singleton or list-enumerated
/// field during deserialization. It is used in the following way:
///
///```
/// use ivms101::ZeroToN;
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Foo {
///     #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
///     bar: ZeroToN<u8>,
/// }
/// ```
///
/// If decorated with these serde attributes, a `ZeroToN` field will be skipped
/// during serialization if it is either `ZeroToN::None` or `ZeroToN::N(vec![])`.
///
/// During deserialization, the `default` tag will cause the field to be
/// deserialized into `ZeroToN::None` if the field is not present.
///
/// As a consequence of the usage of serde attributes, `ZeroToN` cannot be
/// applied to the root deserialization object.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ZeroToN<T> {
    #[default]
    None,
    One(T),
    N(Vec<T>),
}

impl<T> ZeroToN<T> {
    /// Indicates whether any items are present.
    ///
    /// ```
    /// use ivms101::ZeroToN;
    ///
    /// assert!(!ZeroToN::from(Some(8)).is_empty());
    /// assert!(ZeroToN::<u8>::from(None).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            ZeroToN::None => true,
            ZeroToN::One(_) => false,
            ZeroToN::N(v) => v.is_empty(),
        }
    }

    /// Returns a reference to the first element if there is one,
    /// and `None` otherwise.
    ///
    /// ```
    /// use ivms101::ZeroToN;
    ///
    /// assert_eq!(ZeroToN::from(Some(8)).first(), Some(&8));
    /// assert_eq!(ZeroToN::<u8>::from(None).first(), None);
    /// ```
    pub fn first(&self) -> Option<&T> {
        match self {
            ZeroToN::None => None,
            ZeroToN::One(t) => Some(t),
            ZeroToN::N(v) => v.first(),
        }
    }
}

impl<T> IntoIterator for ZeroToN<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            ZeroToN::None => vec![].into_iter(),
            ZeroToN::One(t) => vec![t].into_iter(),
            ZeroToN::N(v) => v.into_iter(),
        }
    }
}

impl<T> From<Option<T>> for ZeroToN<T> {
    fn from(from: Option<T>) -> Self {
        match from {
            Some(t) => ZeroToN::One(t),
            None => ZeroToN::None,
        }
    }
}

impl<T> From<Vec<T>> for ZeroToN<T> {
    fn from(from: Vec<T>) -> Self {
        ZeroToN::N(from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_serialization() {
        #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        struct ZeroToNTest {
            #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
            foo: ZeroToN<u8>,
        }

        assert_tokens(&ZeroToN::<u8>::None, &[Token::Unit]);
        assert_tokens(&ZeroToN::<u8>::One(1), &[Token::U8(1)]);
        assert_tokens(
            &ZeroToN::<u8>::N(vec![1]),
            &[Token::Seq { len: Some(1) }, Token::U8(1), Token::SeqEnd],
        );
        serde_test::assert_ser_tokens(
            &ZeroToNTest {
                foo: ZeroToN::N(vec![]),
            },
            &[
                Token::Struct {
                    name: "ZeroToNTest",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
        serde_test::assert_de_tokens(
            &ZeroToNTest { foo: ZeroToN::None },
            &[
                Token::Struct {
                    name: "ZeroToNTest",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
        serde_test::assert_de_tokens(
            &ZeroToN::<u8>::N(vec![]),
            &[Token::Seq { len: None }, Token::SeqEnd],
        );
    }
}
