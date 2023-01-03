use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ExpandableValue<'a> {
    Array(Vec<ExpandableValue<'a>>),
    Object(Vec<(String, ObjectField<'a>)>),
    Other(Value),
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

    pub fn as_object_mut(&mut self) -> Option<&mut Vec<(String, ObjectField<'a>)>> {
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

    impl From<Value> for ExpandableValue<'_> {
        fn from(value: Value) -> Self {
            match value {
                Value::Array(v) => {
                    ExpandableValue::Array(v.into_iter().map(|e| e.into()).collect())
                }
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
