# tf2-price

Utilities for Team Fortress 2 item pricing.

## Usage

### Basic usage
```rust
use tf2_price::{Currencies, refined, scrap};

let currencies = Currencies {
    keys: 5,
    metal: refined!(2) + scrap!(3),
};

// 2.33 refined - metal values are counted in weapons.
assert_eq!(currencies.metal, 42);

// String conversions
assert_eq!(
    format!("Selling for {currencies}."),
    "Selling for 5 keys, 2.33 ref.",
);
assert_eq!(
    Currencies::try_from("5 keys, 2.33 ref").unwrap(),
    currencies,
);
```

### Arithmetic
```rust
use tf2_price::Currencies;

let golden_frying_pan = Currencies {
    keys: 3000,
    metal: 0,
};

assert_eq!(
    // Multiply by an integer.
    golden_frying_pan * 2,
    Currencies {
        keys: 6000,
        metal: 0,
    },
);
assert_eq!(
    // Multiply by a floating point number.
    golden_frying_pan * 2.5,
    Currencies {
        keys: 7500,
        metal: 0,
    },
);

let other_currencies = Currencies {
    keys: 0,
    metal: 2,
};

assert_eq!(
    // Add another currencies.
    golden_frying_pan + other_currencies,
    Currencies {
        keys: 3000,
        metal: 2,
    },
);

// Helper methods for checking for integer overflow.
let max_keys = Currencies {
    keys: i64::MAX,
    metal: 0,
};

assert_eq!(currencies.checked_add(max_keys), None);
assert_eq!(currencies.checked_mul(i64::MAX), None);
```

### Floating Point Key Values
```rust
use tf2_price::{Currencies, ListingCurrencies};

// To preserve floating point key values, use ListingCurrencies.
let currencies = ListingCurrencies {
    keys: 1.5,
    metal: 0,
};

// Conversions to Currencies are supported.
let listing_currencies = ListingCurrencies {
    keys: 1.0,
    metal: 0
};
assert!(
    Currencies::try_from(listing_currencies).is_ok()
);
// Fails if the key value holds a fractional number.
let listing_currencies = ListingCurrencies {
    keys: 1.5,
    metal: 0
};
assert!(
    Currencies::try_from(listing_currencies).is_err()
);
```

### Serialization
```rust
use tf2_price::Currencies;

// Serde deserialization.
let json = r#"{"keys":5,"metal":2.33}"#;
let currencies: Currencies = serde_json::from_str(json).unwrap();

assert_eq!(
    currencies,
    Currencies {
        keys: 5,
        metal: refined!(2) + scrap!(3),
    },
);
```

## License

[MIT](https://github.com/juliarose/tf2-price/tree/main/LICENSE)
