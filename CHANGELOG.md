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
