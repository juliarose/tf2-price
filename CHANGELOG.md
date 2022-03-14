# Changelog

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

## 0.4.0 (2022-03-02)

### Added
- `get_metal_from_float` helper method.
- `get_metal_float` helper method.
- `std::ops::AddAssign<Currencies>` implementation for `Currencies`.
- `std::ops::SubAssign<Currencies>` implementation for `Currencies`.

## 0.5.0 (2022-03-13)

### Added
- `ListingCurrencies` for currencies which require a float value for keys.
- `from_keys_f32` method for `Currencies`.
- `from_listing_currencies` method for `Currencies`.

### Changed
- `Rounding::Up` is now `Rounding::UpScrap`.
- `Rounding::Down` is now `Rounding::DownScrap`.

## 0.5.1 (2022-03-14)

### Fixed
- Missing borrowed `std::ops` for `Currencies` and `ListingCurrencies`.

## 0.5.2 (2022-03-14)

### Fixed
- Incorrect documentation comments.