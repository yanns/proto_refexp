use serde_json::Value;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let mut products = parse(include_bytes!("../products.json").to_owned().as_mut_slice()).unwrap();

    let product_type = parse(
        include_bytes!("../product-type.json")
            .to_owned()
            .as_mut_slice(),
    )
    .unwrap();
    let category = parse(include_bytes!("../category.json").to_owned().as_mut_slice()).unwrap();

    transform(&mut products, product_type, category);
    let output = serialize(products);
    println!("{}", &output);
}

fn parse(s: &mut [u8]) -> serde_json::Result<Value> {
    serde_json::from_slice(s)
}

fn transform(products: &mut Value, expanded_product_type: Value, expanded_category: Value) {
    if let Some(products_array) = products.as_array_mut() {
        for product in products_array {
            if let Some(product) = product.as_object_mut() {
                if let Some(product_type) = product
                    .get_mut("productType")
                    .and_then(|v| v.as_object_mut())
                {
                    product_type.insert("obj".to_string(), expanded_product_type.clone());
                }

                if let Some(categories) =
                    product.get_mut("categories").and_then(|v| v.as_array_mut())
                {
                    for category in categories {
                        if let Some(category) = category.as_object_mut() {
                            category.insert("obj".to_string(), expanded_category.clone());
                        }
                    }
                }
            }
        }
    }
}

fn serialize(v: Value) -> String {
    v.to_string()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::transform;

    #[test]
    fn transform_add_product_type_and_categories() {
        // given
        let mut products = products();
        let product_type = product_type();
        let category = category();

        // when
        transform(&mut products, product_type, category);

        // then
        assert_eq!(products, expanded_products());
    }

    fn products() -> Value {
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
    }

    fn expanded_products() -> Value {
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
