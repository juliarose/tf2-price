# tf2-price

Utilities for Team Fortress 2 item pricing. Lightweight with [only one required dependency](https://github.com/juliarose/tf2-price/tree/main/Cargo.toml).

Fractional currencies pose arithmetic challenges due to the inherent imprecision of floating-point numbers. A solution is to handle currency in its smallest unit (e.g., cents for US currency, or weapons in Team Fortress 2), stored as integers. This allows precise calculations without [cumbersome conversions](https://gist.github.com/juliarose/f2b5aaa2c71b90d536668e0143d16936), ensuring predictable outcomes. Additionally, this crate offers a container for floating-point currencies when needed.

In release builds in Rust, integers pose the risk of [overflowing](https://en.wikipedia.org/wiki/Integer_overflow). While, this behaviour is [not considered unsafe](https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html#integer-overflow), it is problematic. This crate uses [saturating arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic) for integer arithmetic and also provides methods for checking for overflow.

## Installation

### With Serde
```
tf2-price = { version = "0.12.0", features = ["serde"] }
```

### Without Serde
```
tf2-price = "0.12.0"
```

## Usage

### Basic Usage
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

// 5 (keys) * 50 (key price) * 18 (one ref) = 4500
// 4500 + 24 (weapons) = 4524
assert_eq!(total, 4524);
assert_eq!(
    // Convert total back into keys + weapons.
    Currencies::from_weapons(total, key_price_weapons),
    currencies,
);
```

### Arithmetic
```rust
use tf2_price::{Currencies, Currency};

let golden_pan = Currencies {
    keys: 3000,
    weapons: 0,
};

assert_eq!(
    // Multiply by an integer.
    golden_pan * 2,
    Currencies {
        keys: 6000,
        weapons: 0,
    },
);
assert_eq!(
    // Multiply by a floating point number.
    golden_pan * 2.5,
    Currencies {
        keys: 7500,
        weapons: 0,
    },
);
assert_eq!(
    // Add another currencies.
    golden_pan + Currencies {
        keys: 0,
        weapons: 2,
    },
    Currencies {
        keys: 3000,
        weapons: 2,
    },
);
```

Due to the vast size of 64-bit integers, worries about reaching their bounds are generally unnecessary. To give a practical example:

```rust 
use tf2_price::{Currencies, Currency, refined};

// Let's say a Golden Pan is selling for 3000 keys.
let golden_pan = Currencies {
    keys: 3000,
    weapons: 0,
};
// And the price of a key is 50 refined.
let key_price_weapons = refined!(50); // 900
// And we convert the price of the Golden Pan into weapons.
let golden_pan_weapons = golden_pan.to_weapons(
    key_price_weapons
); // 2700000
// And take how many times this number fits into the max value.
let num_golden_pans = Currency::MAX / golden_pan_weapons;

// It would be enough weapons to buy over 3 trillion Golden Pans.
assert_eq!(num_golden_pans, 3_416_063_717_353);
// We can multiply the number of Golden Pans by the price of one.
assert!(Currencies {
    keys: 0,
    weapons: golden_pan_weapons,
}.checked_mul(num_golden_pans).is_some());
// But if we try to multiply by one more, it will overflow.
assert!(Currencies {
    keys: 0,
    weapons: golden_pan_weapons,
}.checked_mul(num_golden_pans + 1).is_none());
```

### Floating Point Precision

To store original floating point numbers from responses, use `FloatCurrencies` as a container. However, it's advised not to use it for calculations or comparisons. This crate provides utilities for converting floats to integers based on use-case ([saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic), [checked](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/statements/checked-and-unchecked)).
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
