use crate::expandable_value::{ExpandableValue, ObjectField};
use serde::{Serialize, Serializer};

impl Serialize for ExpandableValue<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExpandableValue::Array(v) => v.serialize(serializer),
            ExpandableValue::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len())).unwrap();
                for (k, v) in m {
                    map.serialize_entry(k, &v).unwrap()
                }
                map.end()
            }
            ExpandableValue::Other(v) => v.serialize(serializer),
        }
    }
}

impl Serialize for ObjectField<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ObjectField::Field(v) => v.serialize(serializer),
            ObjectField::ExpandedReference(v) => v.serialize(serializer),
        }
    }
}
