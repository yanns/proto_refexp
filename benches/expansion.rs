use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proto_refexp::expand;

pub fn criterion_benchmark(c: &mut Criterion) {
    let products = include_str!("../products.json");
    let product_type = include_str!("../product-type.json");
    let category = include_str!("../category.json");
    c.bench_function("expand", |b| {
        b.iter(|| {
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
