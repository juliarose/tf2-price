# tf2-price

Utilities for Team Fortress 2 item pricing.

## Usage

Due to issues with floating point precision, it's [common practice to do arithmetic on fixed measurements](https://en.wikipedia.org/wiki/Fixed-point_arithmetic). One way of approaching this problem is to think of currency in its lowest unit. For example, for US currency this is one cent. In Team Fortress 2, this is one weapon.

### Basic usage
```rust
use tf2_price::{Currencies, metal, ONE_REF, ONE_REC};

let currencies = Currencies {
    keys: 5,
    weapons: metal!(1.33), // 24 weapons.
};

// 1.33 refined or 24 weapons.
assert_eq!(currencies.weapons, 24);
assert_eq!(currencies.weapons, ONE_REF + ONE_REC);
assert_eq!(currencies.weapons, metal!(1.33));

// String conversions.
assert_eq!(
    format!("Selling for {currencies}."),
    "Selling for 5 keys, 1.33 ref.",
);
assert_eq!(
    "5 keys, 1.33 ref".parse::<Currencies>().unwrap(),
    currencies,
);
```

### Arithmetic
```rust
use tf2_price::{Currencies, Currency};
    
let golden_frying_pan = Currencies {
    keys: 3000,
    weapons: 0,
};

assert_eq!(
    // Multiply by an integer.
    golden_frying_pan * 2,
    Currencies {
        keys: 6000,
        weapons: 0,
    },
);
assert_eq!(
    // Multiply by a floating point number.
    golden_frying_pan * 2.5,
    Currencies {
        keys: 7500,
        weapons: 0,
    },
);

let other_currencies = Currencies {
    keys: 0,
    weapons: 2,
};

assert_eq!(
    // Add another currencies.
    golden_frying_pan + other_currencies,
    Currencies {
        keys: 3000,
        weapons: 2,
    },
);

// Helper methods for checking for integer overflow.
let currencies = Currencies {
    keys: 2,
    weapons: 0,
};
let max_keys = Currencies {
    keys: Currency::MAX,
    weapons: 0,
};

assert_eq!(currencies.checked_add(max_keys), None);
assert_eq!(currencies.checked_mul(Currency::MAX), None);
```

### Floating Point Precision

Since responses usually contain floating point numbers, we also need a way to store these values. It is recommended to use this only as a container for converting into `Currencies` and not for calculations and comparisons. This crate offers utilities for handling the complicated task of converting floats into integers depending on use-case (saturating, checked).

```rust
use tf2_price::{Currencies, FloatCurrencies, Currency};

// To preserve original values, use FloatCurrencies.
let float_currencies = FloatCurrencies {
    keys: 1.0,
    // Unlike Currencies, metal is not counted in weapons.
    // 1.33 means 1.33 refined.
    metal: 1.33,
};
// Converting to Currencies (checks for safe conversion).
let currencies = Currencies::try_from(float_currencies).unwrap();

assert_eq!(currencies.keys, 1);
// 1.33 refined.
assert_eq!(currencies.weapons, 24);

// Fails if the key value holds a fractional number.
assert!(Currencies::try_from(FloatCurrencies {
    keys: 1.5,
    metal: 0.0,
}).is_err());
// Fails if a value is outside of integer bounds.
assert!(Currencies::try_from(FloatCurrencies {
    keys: Currency::MAX as f32 * 2.0,
    metal: 0.0,
}).is_err());
```

### Serialization
```rust
use tf2_price::{Currencies, metal};

// Serde deserialization.
let json = r#"{"keys":5,"metal":2.33}"#;
let currencies: Currencies = serde_json::from_str(json).unwrap();

assert_eq!(
    currencies,
    Currencies {
        keys: 5,
        weapons: metal!(2.33),
    },
);
```

## License

[MIT](https://github.com/juliarose/tf2-price/tree/main/LICENSE)
