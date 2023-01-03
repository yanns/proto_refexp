use serde_json::Value;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ExpandableValue {
    Array(Vec<ExpandableValue>),
    Object(Vec<(String, ObjectField)>),
    Other(Value),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ObjectField {
    Field(ExpandableValue),
    ExpandedReference(Rc<Value>),
}

impl ExpandableValue {
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<ExpandableValue>> {
        match self {
            ExpandableValue::Array(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Vec<(String, ObjectField)>> {
        match self {
            ExpandableValue::Object(map) => Some(map),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::expandable_value::{ExpandableValue, ObjectField};

    impl From<Value> for ExpandableValue {
        fn from(value: Value) -> Self {
            match value {
                Value::Array(v) => ExpandableValue::Array(v.into_iter().map(|e| e.into()).collect()),
                Value::Object(o) => ExpandableValue::Object(
                    o.into_iter()
                        .map(|kv| (kv.0, ObjectField::Field(kv.1.into())))
                        .collect(),
                ),
                other => ExpandableValue::Other(other),
            }
        }
    }
}
