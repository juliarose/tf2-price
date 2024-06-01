# Changelog

## 0.13.2 (2024-06-01)

### Changed
- Updated readme to be a bit easier to understand.
- Adding missing comment about overflows for `TryFrom<FloatCurrencies>` for `Currencies`.

## 0.13.1 (2024-04-06)

### Fixed
- Readme displaying the wrong version.

## 0.13.0 (2024-04-06)

### Changed
- **MAJOR CHANGE:** Renamed `ListingCurrencies` to `FloatCurrencies`. Both `keys` and `metal` fields are now defined as `f32`. This makes more sense as this is generally used as a container type for holding original values from responses. `FloatCurrencies` should generally be converted to `Currencies` in order to perform arithmetic.
- **MAJOR CHANGE:** Renamed `metal` to `weapons` for `Currencies`. Many values which are in weapons are suffixed with `weapons` to be concise what the value is measured in.
- Changed method names to fit with the above changes.
- Printing `Currencies` and `FloatCurrencies` now prints `"0 keys, 0 ref"` when currencies are empty.
- `TryFrom<FloatCurrencies>` for `Currencies` is strict now.
- `checked_add` and `checked_sub` now take owned values.
- `impl_ops` dependency to `auto_ops`.
- `serde` dependency is now optional.

### Fixed
- Previously, parsing strings with empty currencies e.g. "0 keys" would result in an error. This is no longer the case.

### Added
- `metal!` macro.
- `checked_get_weapons_from_metal_float` helper method.
- `checked_from_weapons` for `Currencies`.
- `Hash` derive for `Currencies`.
- `FromStr` , `TryFrom<&String>`, and `TryFrom<String>` for `Currencies`.
- `FromStr` , `TryFrom<&String>`, and `TryFrom<String>` for `FloatCurrencies`.
- More detailed errors.

### Removed
- `USDCurrencies` struct. This is somewhat out of the scope of this crate and can be better solved with other crates. The `rust_decimal` crate for example is a popular choice.
- `SerializeCurrencies` trait.
- `round` from `FloatCurrencies`.
- `is_fract` from `FloatCurrencies`.

## 0.12.0 (2024-01-26)

### Changed
- Moved `types::Currency` to root.
- Added feature `b32` to use currencies defined as i32 rather than i64 when enabled.

## 0.11.0 (2023-02-17)

### Changed
- **MAJOR CHANGE:** All integer types are now `i64`. `i32` may not be big enough for some use-cases. For example: expressing the value of all Golden Pans in existence as number of weapons reaches close to the bounds. Given a key price of 80 refined and a Golden Pan price of 3400 keys, `i32` can only hold value for 438, while of course `i64` is exponentially larger and can hold value for 1883858667658 allowing more flexibility.
- `checked_add` on `Currencies` is now under `impl` rather than a trait.
- `checked_add` on `FloatCurrencies` is now under `impl` rather than a trait.

### Added
- `Currency` type to represent the primitive used for integers.

### Removed
- `ParseError::NoCurrencies`.

## 0.10.0 (2023-02-16)

### Added
- `neaten` to `Currencies`.

### Changed
- `round` on `Currencies` returns `self`.
- `round` on `FloatCurrencies` returns `self`.

### Fixed
- An issue with overflowing values resulting in `i32::MIN` in the `to_metal` method.

## 0.9.0 (2023-02-01)

### Added
- `can_afford` to `Currencies`.
- `can_afford` to ``FloatCurrencies``.

### Changed
- Aside from the macros, all arithmatic operations are now saturating.

## 0.8.1 (2023-01-14)

### Added
- `checked_to_metal` to `Currencies`.
- `checked_to_metal` to ``FloatCurrencies``.

## 0.8.0 (2022-12-31)

### Added
- `checked_add` to `Currencies`.
- `checked_sub` to `Currencies`.
- `checked_mul` to `Currencies`.
- `checked_div` to `Currencies`.
- `checked_add` to `USDCurrencies`.
- `checked_sub` to `USDCurrencies`.
- `checked_mul` to `USDCurrencies`.
- `checked_div` to `USDCurrencies`.

## 0.7.2 (2022-09-22)

### Fixed
- `to_metal` in `Currencies` method overflowing or underflowing with addition or multiplication when working with larger values.

## 0.7.1 (2022-05-11)

### Fixed
- `to_metal` in `FloatCurrencies` method not rounding values.

## 0.7.0 (2022-05-04)

### Added
- `USDCurrencies` for currencies which hold a cash value.

## 0.6.0 (2022-04-21)

### Added
- `SerializeCurrencies` implementation for `Currencies` and `FloatCurrencies`.
- `Ord` implementation for `Currencies` and `FloatCurrencies`.

## 0.5.3 (2022-03-18)

### Fixed
- `fmt::Display` implementation for `Currencies` and `FloatCurrencies` not recognizing negative numbers.

### Changed
- `fmt::Display` implementation for `Currencies` and `FloatCurrencies` now displays `"nothing"` when currencies are empty. 

### Added
- `std::ops::MulAssign<i32>` implementation for `Currencies`.
- `std::ops::MulAssign<f32>` implementation for `Currencies`.
- `std::ops::DivAssign<i32>` implementation for `Currencies`.
- `std::ops::DivAssign<f32>` implementation for `Currencies`.
- `std::ops::MulAssign<i32>` implementation for `FloatCurrencies`.
- `std::ops::MulAssign<f32>` implementation for `FloatCurrencies`.
- `std::ops::DivAssign<i32>` implementation for `FloatCurrencies`.
- `std::ops::DivAssign<f32>` implementation for `FloatCurrencies`.

## 0.5.2 (2022-03-14)

### Fixed
- Incorrect documentation comments.

## 0.5.1 (2022-03-14)

### Fixed
- Missing borrowed `std::ops` for `Currencies` and `FloatCurrencies`.

## 0.5.0 (2022-03-13)

### Added
- `FloatCurrencies` for currencies which require a float value for keys.
- `from_keys_f32` method for `Currencies`.
- `from_float_currencies` method for `Currencies`.

### Changed
- `Rounding::Up` is now `Rounding::UpScrap`.
- `Rounding::Down` is now `Rounding::DownScrap`.

## 0.4.0 (2022-03-02)

### Added
- `get_metal_from_float` helper method.
- `get_metal_float` helper method.
- `std::ops::AddAssign<Currencies>` implementation for `Currencies`.
- `std::ops::SubAssign<Currencies>` implementation for `Currencies`.

## 0.3.0 (2022-02-20)

### Added
- `Currencies.from_metal` associated function.
- `std::ops::Add<Currencies>` implementation for `Currencies`.
- `std::ops::Sub<Currencies>` implementation for `Currencies`.
- `std::ops::Div<i32>` implementation for `Currencies`.
- `std::ops::Mul<i32>` implementation for `Currencies`.

### Changed
- `Currencies.to_value` method is now `Currencies.to_metal`.
- `Currencies` uses i32 instead of u32.

## 0.2.0 (2022-02-14)

### Added
- `RoundMetal::Refined` to round to nearest refined. 
- `RoundMetal::UpRefined` to round up to nearest refined. 
- `RoundMetal::DownRefined` to round down to nearest refined. 

### Changed
- `Currencies` can now serialize/deserialize into the proper format now.
- `Currencies.empty` method is now `Currencies.is_empty`.
- `Currencies.round` now only accepts a borrowed value of `RoundMetal`.
- Moderate performance enhancements for `std::fmt::Display` implementation of `Currencies`.
  
### Removed
- `CurrenciesForm` struct, as `Currencies` can now serialize/deserialize into the proper format now.
