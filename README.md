# tf2-price

Utilities for Team Fortress 2 item pricing.

Fractional currencies pose arithmetic challenges due to the inherent imprecision of floating-point numbers. A solution is to handle currency in its smallest unit (e.g., cents for US currency, or weapons in Team Fortress 2), stored as integers. This allows precise calculations without cumbersome conversions, ensuring predictable outcomes. Additionally, this crate offers a container for floating-point currencies when needed.

## Installation

### Cargo.toml
```
tf2-price = "0.12.0"
```

## Usage

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

// Key price stored as weapons.
let key_price_weapons = metal!(50);
// Conversion to a single total.
let total = currencies.to_weapons(key_price_weapons);

assert_eq!(total, 924);
assert_eq!(
    // Convert total back into keys + weapons.
    Currencies::from_weapons(total, key_price_weapons),
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

To store original floating point numbers from responses, use `FloatCurrencies` as a container. However, it's advised not to use it for calculations or comparisons. This crate provides utilities for converting floats to integers based on use-case (saturating, checked).

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

assert_eq!(
    currencies,
    Currencies {
        keys: 1,
        metal: 24,
    },
);

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
