use proto_refexp::expand;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let products = include_str!("../products.json");
    let product_type = include_str!("../product-type.json");
    let category = include_str!("../category.json");

    let output = expand(products, product_type, category);

    println!("{}", &output);
}
