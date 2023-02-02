use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use fun_stuff::type_session::{view_stock, ViewStock};

#[cfg(feature = "blocking")]
pub fn view_stock_bench(c: &mut Criterion) {
    c.bench_function("view stock", |b| {
        b.iter(|| {
            let mut chan = ViewStock;
            view_stock(chan);
            todo!()
        })
    });
}

criterion_group!(benches, view_stock_bench);
criterion_main!(benches);
