# tf2-price

Price formatting for Team Fortress 2 items.

```rs
use tf2_price::Currencies;

let currencies = Currencies {
    keys: 5,
    // metal values are counted in weapons
    metal: 42,
};

println!("{}", currencies);
// "5 keys, 2.33 ref"

let currencies = Currencies::try_from("5 keys, 2.33 ref").unwrap();
// Currencies {
//     keys: 5,
//     metal: 42,
// }
```

## License

MIT