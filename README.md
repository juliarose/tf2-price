# tf2-price

Price formating for Team Fortress 2 items.

```rs
use tf2_price::Currencies;

let currencies = Currencies {
    keys: 5,
    metal: 36,
};

println!("{}", currencies);
// "5 keys, 2 ref"
```

## License

MIT