use serde_json::Value;
use std::borrow::Cow;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ExpandableValue<'a> {
    Array(Vec<ExpandableValue<'a>>),
    Object(Vec<(Cow<'a, str>, ObjectField<'a>)>),
    Other(Value),
    String(Cow<'a, str>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ObjectField<'a> {
    Field(ExpandableValue<'a>),
    ExpandedReference(&'a Value),
}

impl<'a> ExpandableValue<'a> {
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<ExpandableValue<'a>>> {
        match self {
            ExpandableValue::Array(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Vec<(Cow<'a, str>, ObjectField<'a>)>> {
        match self {
            ExpandableValue::Object(map) => Some(map),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expandable_value::{ExpandableValue, ObjectField};
    use serde_json::Value;
    use std::borrow::Cow;

    impl From<Value> for ExpandableValue<'_> {
        fn from(value: Value) -> Self {
            match value {
                Value::Array(v) => {
                    ExpandableValue::Array(v.into_iter().map(|e| e.into()).collect())
                }
                Value::Object(o) => ExpandableValue::Object(
                    o.into_iter()
                        .map(|kv| (Cow::Owned(kv.0), ObjectField::Field(kv.1.into())))
                        .collect(),
                ),
                Value::String(s) => ExpandableValue::String(Cow::Owned(s)),
                other => ExpandableValue::Other(other),
            }
        }
    }
}
