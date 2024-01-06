# tf2-price

Utilities for Team Fortress 2 item pricing.

## Usage

```rs
use tf2_price::{Currencies, ListingCurrencies, refined, scrap};

let currencies = Currencies { keys: 5, metal: refined!(2) + scrap!(3) };

// 2.33 refined - metal values are counted in weapons.
assert_eq!(currencies.metal, 42);

// String conversions.
assert_eq!(currencies.to_string(), "5 keys, 2.33 ref");
assert_eq!(Currencies::try_from("5 keys, 2.33 ref").unwrap(), currencies);

// Serde deserialization.
let json = r#"{"keys":5,"metal":2.33}"#;
let currencies: Currencies = serde_json::from_str(json).unwrap();

assert_eq!(currencies, Currencies { keys: 5, metal: refined!(2) + scrap!(3) });

// Arithmetic.
let golden_frying_pan = Currencies { keys: 3000, metal: 0 };

assert_eq!(golden_frying_pan * 2, Currencies { keys: 6000, metal: 0 });
assert_eq!(golden_frying_pan * 2.5, Currencies { keys: 7500, metal: 0 });
assert_eq!(
    golden_frying_pan + Currencies { keys: 0, metal: 2 },
    Currencies { keys: 3000, metal: 2 },
);

// There are also some helper methods for checking for integer overflow.
assert_eq!(currencies.checked_add(Currencies { keys: i64::MAX, metal: 0 }), None);
assert_eq!(currencies.checked_mul(i64::MAX), None);

// For currencies which require floating point key values, use ListingCurrencies.
let currencies = ListingCurrencies { keys: 1.5, metal: 0 };

// Conversions to Currencies are supported.
assert!(Currencies::try_from(ListingCurrencies { keys: 1.0, metal: 0 }).is_ok());
// Fails if the key value holds a fractional number.
assert!(Currencies::try_from(ListingCurrencies { keys: 1.5, metal: 0 }).is_err());
```

## License

[MIT](https://github.com/juliarose/tf2-price/tree/main/LICENSE)
