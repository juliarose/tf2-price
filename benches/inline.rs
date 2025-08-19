use criterion::{criterion_group, criterion_main, Criterion};
use tf2_price::{Currencies, FloatCurrencies, Rounding};

fn criterion_benchmark(c: &mut Criterion) {
    let currencies = Currencies {
        keys: 12,
        weapons: 2,
    };
    let rhs = Currencies {
        keys: 10,
        weapons: 3,
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
    
    c.bench_function("from_keys_f32", |b| b.iter(||
        Currencies::from_keys_f32(1.5, key_price)
    ));
    
    c.bench_function("to_weapons", |b| b.iter(||
        currencies.to_weapons(key_price)
    ));
    
    c.bench_function("checked_to_weapons", |b| b.iter(||
        currencies.checked_to_weapons(key_price)
    ));
    
    c.bench_function("is_empty", |b| b.iter(||
        currencies.is_empty()
    ));
    
    c.bench_function("round", |b| b.iter(||
        currencies.round(&Rounding::UpScrap)
    ));
    
    c.bench_function("neaten", |b| b.iter(||
        currencies.neaten(key_price)
    ));
    
    c.bench_function("can_afford", |b| b.iter(||
        currencies.can_afford(&rhs)
    ));
    
    c.bench_function("checked_mul", |b| b.iter(||
        currencies.checked_mul(5)
    ));
    
    c.bench_function("checked_div", |b| b.iter(||
        currencies.checked_div(5)
    ));
    
    c.bench_function("checked_add", |b| b.iter(||
        currencies.checked_add(rhs)
    ));
    
    c.bench_function("checked_sub", |b| b.iter(||
        currencies.checked_add(rhs)
    ));
    
    c.bench_function("FromStr", |b| b.iter(||
        "10 keys, 2 ref".parse::<Currencies>()
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);