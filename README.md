# tf2-price

Price formatting for Team Fortress 2 items.

```rs
use tf2_price::{Currencies, ListingCurrencies, refined, scrap};

fn main() {
    let currencies = Currencies {
        keys: 5,
        metal: refined!(2) + scrap!(3),
    };
    
    // 2.33 refined - metal values are counted in weapons
    assert_eq!(currencies.metal, 42);
    
    // String conversions
    assert_eq!(currencies.to_string(), "5 keys, 2.33 ref");
    assert_eq!(Currencies::try_from("5 keys, 2.33 ref").unwrap(), *&currencies);
    
    // Serde deserialization
    assert_eq!(serde_json::from_str::<Currencies>(r#"{"keys":5,"metal":2.33}"#).unwrap(), *&currencies);
    
    let golden_frying_pan = Currencies { keys: 3000, metal: 0 };
    
    // Arithmetic
    assert_eq!(&golden_frying_pan * 2, Currencies { keys: 6000, metal: 0 });
    assert_eq!(&golden_frying_pan * 2.5, Currencies { keys: 7500, metal: 0 });
    assert_eq!(&golden_frying_pan + Currencies { keys: 0, metal: 2 }, Currencies { keys: 3000, metal: 2 });
    
    // For currencies which require floating point key values, use ListingCurrencies
    let currencies = ListingCurrencies {
        keys: 1.5,
        metal: 0,
    };
    
    // Arithmetic with standard Currencies objects still works
    assert_eq!(&currencies - Currencies { keys: 1, metal: 0 }, ListingCurrencies { keys: 0.5, metal: 0 });
    
    // Conversions to Currencies are supported but will fail if the key value holds a fractional number
    assert_eq!(Currencies::try_from(&currencies).is_err(), true);
    
    // Works fine
    assert_eq!(Currencies::try_from(ListingCurrencies { keys: 1.0, metal: 0 }).is_ok(), true);
}
```

## License

MIT