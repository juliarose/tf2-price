# tf2-price

Provides utilities for Team Fortress 2 item pricing.

## Installation with Serde

```toml
[dependencies]
tf2-price = { version = "0.13", features = ["serde"] }
```

## Usage

### Basic Usage

To avoid issues with [floating point arithmetic](https://en.wikipedia.org/wiki/Floating-point_arithmetic#Accuracy_problems), values are stored as 64-bit integers. For metal, this means using the lowest denomination of currency which is weapons.

```rust
use tf2_price::{Currencies, metal};

let currencies = Currencies {
    keys: 5,
    weapons: metal!(1.33), // 24 weapons.
};

// String conversions.
println!("Selling for {currencies}."); // Selling for 5 keys, 1.33 ref.

let currencies = "5 keys, 1.33 ref".parse::<Currencies>().unwrap();

// Key price stored as weapons.
let key_price_weapons = metal!(50);
// Conversion to a single total.
let total = currencies.to_weapons(key_price_weapons); // 4524 weapons.
// Convert total back into keys + weapons.
let currencies = Currencies::from_weapons(total, key_price_weapons);
```

### Arithmetic

In release builds, integers pose the risk of [overflowing](https://en.wikipedia.org/wiki/Integer_overflow). While this behavior is [not considered unsafe](https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html#integer-overflow), it is problematic. This crate uses [saturating arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic) for integer arithmetic and also provides methods for checking for overflow (using methods such as [`checked_from_weapons`](https://docs.rs/tf2-price/latest/tf2_price/struct.Currencies.html#method.checked_from_weapons)).

```rust
use tf2_price::{Currencies, Currency};

let golden_pan = Currencies {
    keys: 3000,
    weapons: 0,
};

// Multiply by an integer.
let doubled = golden_pan * 2; // Currencies { keys: 6000, weapons: 0 }

// Multiply by a floating point number.
let multiplied = golden_pan * 2.5; // Currencies { keys: 7500, weapons: 0 }

// Add another currencies.
let with_weapons = golden_pan + Currencies {
    keys: 0,
    weapons: 2,
}; // Currencies { keys: 3000, weapons: 2 }
```

### Serialization

While `Currencies` uses `weapons` as the unit for metal, it is converted to and from `metal` when serialized for compatibility with APIs.

```rust
use tf2_price::{Currencies, metal};
    
let json = r#"{"keys":5,"metal":2.33}"#;
let currencies: Currencies = serde_json::from_str(json).unwrap();

assert_eq!(currencies, Currencies { keys: 5, weapons: metal!(2.33) });
assert_eq!(json, serde_json::to_string(&currencies).unwrap());
```

### Floating Point Precision

To store floating point numbers from responses, use `FloatCurrencies` as a container. However, it's advised not to use it for calculations or comparisons. This crate provides utilities for converting floats to integers based on use-case ([saturating](https://en.wikipedia.org/wiki/Saturation_arithmetic), [checked](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/statements/checked-and-unchecked)).

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

assert_eq!(currencies, Currencies { keys: 1, metal: 24 });
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

## License

[MIT](https://github.com/juliarose/tf2-price/tree/main/LICENSE)
