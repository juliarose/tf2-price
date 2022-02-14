# Changelog

## 0.2.0 (2022-04-14)

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