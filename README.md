# tilejson

[![Build Status](https://github.com/georust/tilejson/workflows/Run%20tests/badge.svg)](https://github.com/georust/tilejson/actions)
[![tilejson on crates.io](https://img.shields.io/crates/v/tilejson.svg)](https://crates.io/crates/tilejson)
[![API Docs](https://docs.rs/tilejson/badge.svg)](https://docs.rs/tilejson)

`tilejson` is a crate for serializing/deserializing the [TileJSON](https://github.com/mapbox/tilejson-spec) format — an open standard for representing map metadata.

## Examples

### Reading

```rust
use tilejson::TileJSON;

fn main() {
    let tilejson_str = r#"{
        "tilejson": "3.0.0",
        "name": "compositing",
        "scheme": "tms",
        "tiles": [
            "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"
        ]
    }"#;

    // Parse JSON
    let mut tilejson: TileJSON = serde_json::from_str(&tilejson_str).unwrap();
    println!("{tilejson:?}");
    
    // Add missing default values per TileJSON specification
    tilejson.set_missing_defaults();
    println!("{tilejson:?}");
}
```

### Writing

```rust
use tilejson::tilejson;

fn main() {
    let tilejson = tilejson! {
        "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png".to_string(),
        name: "tileset name".to_string(),
        description: "some description".to_string(),
    };

    let serialized_tilejson = serde_json::to_string(&tilejson).unwrap();

    println!("{serialized_tilejson}");
}
```

## Contributing

Contributions are welcome! Have a look at the [issues](https://github.com/georust/tilejson/issues), and open a pull request if you'd like to add an algorithm or some functionality.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
