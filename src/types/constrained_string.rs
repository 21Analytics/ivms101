#[macro_export]
macro_rules! constrained_string {
    ($newtype:ident, $len_check:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(into = "String", try_from = "&str")]
        pub struct $newtype {
            inner: String,
        }

        impl TryFrom<&str> for $newtype {
            type Error = Error;
            fn try_from(from: &str) -> Result<Self, Error> {
                if $len_check(from.len()) {
                    Ok(Self { inner: from.into() })
                } else {
                    Err(format!(
                        "Cannot parse String of length {} into a {:?}",
                        from.len(),
                        std::any::type_name::<Self>()
                    )
                    .as_str()
                    .into())
                }
            }
        }

        impl From<$newtype> for String {
            fn from(value: $newtype) -> Self {
                value.inner
            }
        }

        impl $newtype {
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.inner
            }
        }

        impl std::fmt::Display for $newtype {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::messages::Error;

    #[test]
    fn test_max_string() {
        crate::constrained_string!(StringMax4, |l| l <= 4);

        let max4 = StringMax4::try_from("0123").unwrap();
        serde_test::assert_tokens(&max4, &[serde_test::Token::BorrowedStr("0123")]);
        assert_eq!(max4.as_str(), "0123");

        serde_test::assert_de_tokens_error::<StringMax4>(
            &[serde_test::Token::BorrowedStr("01234")],
            r#"Validation error: Cannot parse String of length 5 into a "ivms101::types::constrained_string::tests::test_max_string::StringMax4""#,
        );
    }
}
