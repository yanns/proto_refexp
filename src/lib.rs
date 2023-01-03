mod de;
mod expandable_value;
mod ser;

use crate::expandable_value::{ExpandableValue, ObjectField};
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

pub fn expand(products_raw: &str, product_type_raw: &str, category_raw: &str) -> String {
    let mut products = parse_expandable(products_raw).unwrap();
    let product_type = parse(product_type_raw).unwrap();
    let category = parse(category_raw).unwrap();

    transform(&mut products, &product_type, &category);

    serialize(products)
}

fn parse_expandable(s: &str) -> serde_json::Result<ExpandableValue> {
    serde_json::from_str(s)
}

fn parse(s: &str) -> serde_json::Result<Value> {
    serde_json::from_str(s)
}

const OBJ: Cow<str> = Cow::Borrowed("obj");

fn transform<'a>(
    products: &mut ExpandableValue<'a>,
    expanded_product_type: &'a Value,
    expanded_category: &'a Value,
) {
    if let Some(products_array) = products.as_array_mut() {
        for product in products_array {
            if let Some(product_fields) = product.as_object_mut() {
                for (k, v) in product_fields {
                    match v {
                        ObjectField::Field(f) => {
                            if k == "productType" {
                                if let Some(product_type) = f.as_object_mut() {
                                    product_type.push((
                                        OBJ,
                                        ObjectField::ExpandedReference(expanded_product_type),
                                    ));
                                }
                            } else if k == "categories" {
                                if let Some(categories_array) = f.as_array_mut() {
                                    for category in categories_array {
                                        if let Some(category) = category.as_object_mut() {
                                            category.push((
                                                OBJ,
                                                ObjectField::ExpandedReference(expanded_category),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        ObjectField::ExpandedReference(_) => {}
                    }
                }
            }
        }
    }
}

fn serialize<T>(v: T) -> String
where
    T: Serialize,
{
    serde_json::to_string(&v).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::expandable_value::ExpandableValue;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    use crate::transform;

    #[test]
    fn transform_add_product_type_and_categories() {
        // given
        let mut products = products();
        let product_type = product_type();
        let category = category();

        // when
        transform(&mut products, &product_type, &category);

        // then
        assert_eq!(
            serde_json::to_string(&products).unwrap(),
            serde_json::to_string(&expanded_products()).unwrap()
        );
    }

    fn products() -> ExpandableValue<'static> {
        json!([
            {
                "id": "id1",
                "productType": {
                    "typeId": "product-type",
                    "id": "product-type-1"
                },
                "categories": [
                    { "typeId": "category", "id": "cat-1" },
                    { "typeId": "category", "id": "cat-2" },
                    { "typeId": "category", "id": "cat-3" }
                ]
            },
            {
                "id": "id2",
                "productType": {
                    "typeId": "product-type",
                    "id": "product-type-1"
                },
                "categories": [
                    { "typeId": "category", "id": "cat-1" },
                    { "typeId": "category", "id": "cat-4" }
                ]
            }
        ])
        .try_into()
        .unwrap()
    }

    fn expanded_products() -> ExpandableValue<'static> {
        json!([
            {
                "id": "id1",
                "productType": {
                    "typeId": "product-type",
                    "id": "product-type-1",
                    "obj": product_type()
                },
                "categories": [
                    { "typeId": "category", "id": "cat-1", "obj": category() },
                    { "typeId": "category", "id": "cat-2", "obj": category() },
                    { "typeId": "category", "id": "cat-3", "obj": category() }
                ]
            },
            {
                "id": "id2",
                "productType": {
                    "typeId": "product-type",
                    "id": "product-type-1",
                    "obj": product_type()
                },
                "categories": [
                    { "typeId": "category", "id": "cat-1", "obj": category() },
                    { "typeId": "category", "id": "cat-4", "obj": category() }
                ]
            }
        ])
        .into()
    }

    fn product_type() -> Value {
        json!({
            "id": "product-type-id",
            "key": "product-type-key"
        })
    }

    fn category() -> Value {
        json!({
            "id": "cat-id",
            "key": "cat-key"
        })
    }
}
