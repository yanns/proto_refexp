use crate::expandable_value::{ExpandableValue, ObjectField};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt::Formatter;

impl<'de> Deserialize<'de> for ExpandableValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExpandableValueVisitor)
    }
}

struct ExpandableValueVisitor;

impl<'de> Visitor<'de> for ExpandableValueVisitor {
    type Value = ExpandableValue;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("any JSON value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
        Ok(ExpandableValue::Other(Value::Bool(v)))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        Ok(ExpandableValue::Other(Value::Number(v.into())))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(ExpandableValue::Other(Value::String(v.to_string())))
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<ExpandableValue, V::Error>
        where
            V: SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(ExpandableValue::Array(vec))
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> Result<ExpandableValue, V::Error>
        where
            V: MapAccess<'de>,
    {
        let mut v: Vec<(String, ObjectField)> =
            Vec::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some((key, value)) = visitor.next_entry::<String, ExpandableValue>()? {
            v.push((key, ObjectField::Field(value)));
        }

        Ok(ExpandableValue::Object(v))
    }
}

#[cfg(test)]
mod tests {
    use crate::expandable_value::ExpandableValue;
    use indoc::indoc;
    use serde_json::json;

    #[test]
    fn parse_array_of_objects() {
        // given
        let json = indoc! {r#"
            [
                {
                    "name": "Dupont",
                    "age": 42,
                    "hobbies": ["cricket", "ice cream"]
                }
            ]
        "#};

        // when
        let result: ExpandableValue = serde_json::from_str(json).unwrap();

        // then
        let expected: ExpandableValue = json!([
            {
                "name": "Dupont",
                "age": 42,
                "hobbies": ["cricket", "ice cream"]
            }
        ])
        .into();
        assert_eq!(result, expected);
    }
}
