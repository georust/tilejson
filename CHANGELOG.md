# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.3](https://github.com/georust/tilejson/compare/v0.4.2...v0.4.3) - 2025-06-16
- migrate to release-plz CI and rework CI/dependabot pipeline ([#43](https://github.com/georust/tilejson/pull/43), [#36](https://github.com/georust/tilejson/pull/36), [#35](https://github.com/georust/tilejson/pull/35), [#33](https://github.com/georust/tilejson/pull/33), [#32](https://github.com/georust/tilejson/pull/32), [#31](https://github.com/georust/tilejson/pull/31), [#30](https://github.com/georust/tilejson/pull/30))

<a name="v0.4.2"></a>
### v0.4.2 (2025-03-03)
* Update dependencies, set MSRV to 1.78, and some internal cleanup

<a name="v0.4.1"></a>
### v0.4.1 (2022-12-08)
* Add `Bounds::from` for `(f64, f64, f64, f64)` tuple. Same for `f32` and `i32`.
* Add `Center::from` for `(f64, f64, u8)` and `(f32, f32, u8)` tuples.
* A few clippy-related fixes

<a name="v0.4.0"></a>
### v0.4.0 (2022-11-19)
* Switch all `HashMap` to `BTreeMap` for consistent serialization ordering

<a name="v0.3.4"></a>
### v0.3.4 (2022-11-15)
* Add proper `Error` implementation to `Bounds` and `Center` parsing errors

<a name="v0.3.3"></a>
### v0.3.3 (2022-11-07)
* Add `Display` with precision support for `Bounds` and `Center` structs

<a name="v0.3.2"></a>
### v0.3.2 (2022-10-30)
* Add `Bounds::from` for `[f64; 4]`, `[f32; 4]`, `[i32; 4]`
* Add `Bounds::try_from` now also supports `&[f64]`, `&[f32]`, `&[i32]` in addition to `Vec<f64>`

<a name="v0.3.1"></a>
### v0.3.1 (2022-05-29)
* Add `Bounds::MAX` to create a maximum -180..180, -90..90 value.
* Add `Bounds::MAX_TILED` to create a maximum allowed for vector tiles per spec.
* Implement `Add` and `AddAssign` on `Bounds`

<a name="v0.3.0"></a>
### v0.3.0 (2022-05-25)

**ATTENTION:** This release contains many breaking changes. See [README](README.md) for usage examples.

* Migrate to Rust 2021 edition
* update docs to match v3.0.0 spec
* add `fillzoom` field per v3.0.0 spec
* add `Center` and `Bounds` structs instead of arrays
  * both support `FromStr` trait
* add `VectorLayer` struct and the `vector_layer` field
* Remove builder pattern because `TileJSON` is writable
* Add `other` fields for any unknown fields in root and vector layers
* Restructure instantiation:
  * use `tilejson!{ source }` macro to create `TileJSON` objects, with any number of the optional `field: value` pairs.
  * use `set_missing_defaults()` to replace all missing values with their defaults (only if the spec defines it)
* Remove `id` field because it is not supported by the spec

<a name="v0.2.4"></a>
### v0.2.4 (2021-10-11)


#### Bug Fixes

*   use String instead of &'static str for `tilejson` field (#7) ([25b325c9](https://github.com/georust/tilejson/commit/25b325c9f0618f1cad16899385f87339ac366e20))


<a name="v0.2.3"></a>
### v0.2.3 (2021-10-10)


#### Bug Fixes

*   skip serializing if Option is None (#6) (h/t @jaspervercnocke) ([149339cd](https://github.com/georust/tilejson/commit/149339cd83d9065800c73174b0db1ec0a3465513))



<a name="0.2.2"></a>
### v0.2.2 (2020-09-12)


#### Features

*   change type of bounds to f32 instead of i32 (#1) (h/t @jaspervercnocke) ([a7cffa81](https://github.com/georust/tilejson/commit/a7cffa8181accd3268b8ea96ae2668b24ae016a4))

#### Bug Fixes

*   use crate pub ([09f051a9](https://github.com/georust/tilejson/commit/09f051a901bb5648a9bcce05f12c8fdece7b81c9))
