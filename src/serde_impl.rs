use serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};
use serde::{Serialize, Serializer};
use std::fmt;
use InlinableString;

impl Serialize for InlinableString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl<'de> Deserialize<'de> for InlinableString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InlinableStringVisitor;

        impl<'de> Visitor<'de> for InlinableStringVisitor {
            type Value = InlinableString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v.into())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(v.into())
            }
        }

        deserializer.deserialize_str(InlinableStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    use InlinableString;

    #[test]
    fn test_ser_de() {
        let s = InlinableString::from("small");

        assert_tokens(&s, &[Token::String("small")]);
    }
}
