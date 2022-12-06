use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proto_refexp::expand;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("expand", |b| {
        b.iter(|| {
            let mut products = include_bytes!("../products.json").to_owned();
            let products = products.as_mut();
            let mut product_type = include_bytes!("../product-type.json").to_owned();
            let product_type = product_type.as_mut();
            let mut category = include_bytes!("../category.json").to_owned();
            let category = category.as_mut();

            expand(
                black_box(products),
                black_box(product_type),
                black_box(category),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
