use criterion::{criterion_group, criterion_main, Criterion};
use tf2_price::{Currencies, FloatCurrencies, Rounding};

fn criterion_benchmark(c: &mut Criterion) {
    let mut a = Currencies {
        keys: 12,
        weapons: 2,
    };
    let b = Currencies {
        keys: 5,
        weapons: 1,
    };
    let key_price = 18 * 50;
    let float_currencies = FloatCurrencies {
        keys: 1.5,
        metal: 3.33,
    };
    
    c.bench_function("new", |b| b.iter(||
        Currencies::new()
    ));
    
    c.bench_function("from_float_currencies_with", |b| b.iter(||
        Currencies::from_float_currencies_with(float_currencies, key_price)
    ));
    
    c.bench_function("try_from_float_currencies_with", |b| b.iter(||
        Currencies::try_from_float_currencies_with(float_currencies, key_price)
    ));
    
    c.bench_function("to_weapons", |b| b.iter(||
        a.to_weapons(key_price)
    ));
    
    c.bench_function("checked_to_weapons", |b| b.iter(||
        a.checked_to_weapons(key_price)
    ));
    
    c.bench_function("is_empty", |b| b.iter(||
        a.is_empty()
    ));
    
    c.bench_function("round", |b| b.iter(||
        a.round(Rounding::UpScrap)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);