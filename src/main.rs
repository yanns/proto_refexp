use proto_refexp::expand;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let mut products = include_bytes!("../products.json").to_owned();
    let products = products.as_mut();
    let mut product_type = include_bytes!("../product-type.json").to_owned();
    let product_type = product_type.as_mut();
    let mut category = include_bytes!("../category.json").to_owned();
    let category = category.as_mut();

    let output = expand(products, product_type, category);

    println!("{}", &output);
}
