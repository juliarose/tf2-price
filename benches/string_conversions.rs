use criterion::{criterion_group, criterion_main, Criterion};
use tf2_price::{Currencies, refined, scrap};

fn criterion_benchmark(c: &mut Criterion) {
    let currencies_str_keys_and_ref = "12 keys, 23.33 ref";
    let currencies_keys_and_ref = Currencies {
        keys: 12, 
        metal: refined!(23) + scrap!(3),
    };
    let curencies_keys = Currencies {
        keys: 12,
        metal: 0,
    };
    
    c.bench_function("from string keys and ref", |b| b.iter(||
        Currencies::try_from(currencies_str_keys_and_ref)
    ));
    
    c.bench_function("to string keys and ref", |b| b.iter(||
        format!("{}", currencies_keys_and_ref)
    ));
    
    c.bench_function("to string keys", |b| b.iter(||
        format!("{}", curencies_keys)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}
criterion_main!(benches);