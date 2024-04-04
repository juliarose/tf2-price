use criterion::{criterion_group, criterion_main, Criterion};
use tf2_price::{Currencies, Currency};

fn criterion_benchmark(c: &mut Criterion) {
    let left: Currency = 100;
    let right: Currency = 400;
    let left_currencies = Currencies { keys: 1, metal: 10 };
    let right_currencies = Currencies { keys: 1, metal: 10 };
    
    c.bench_function("adds two numbers", |b| b.iter(||
        left + right
    ));
    
    c.bench_function("saturating adds two numbers", |b| b.iter(||
        left.saturating_add(right)
    ));
    
    c.bench_function("saturating adds two currencies", |b| b.iter(||
        // these are saturating
        left_currencies + right_currencies
    ));
    
    c.bench_function("checked adds two currencies", |b| b.iter(||
        // this checks bounds
        left_currencies.checked_add(right_currencies)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);