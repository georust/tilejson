# tilejson

[![Build Status](https://github.com/georust/tilejson/workflows/Run%20tests/badge.svg)](https://github.com/georust/tilejson/actions)
[![tilejson on crates.io](https://img.shields.io/crates/v/tilejson.svg)](https://crates.io/crates/tilejson)
[![API Docs](https://docs.rs/tilejson/badge.svg)](https://docs.rs/tilejson)

`tilejson` is a crate for serializing/deserializing the [TileJSON](https://github.com/mapbox/tilejson-spec) format — an open standard for representing map metadata.

## Examples

### Reading

```rust
extern crate tilejson;
extern crate serde_json;

use tilejson::TileJSON;

fn main() {
    let tilejson_str = r#"{
        "tilejson": "2.2.0",
        "name": "compositing",
        "scheme": "tms",
        "tiles": [
            "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"
        ]
    }"#;

    let tilejson: TileJSON = serde_json::from_str(&tilejson_str).unwrap();
    println!("{:?}", tilejson);
}
```

### Writing

Using builder pattern

```rust
extern crate tilejson;
extern crate serde_json;

use tilejson::TileJSONBuilder;

fn main() {
    let mut tilejson_builder = TileJSONBuilder::new();

    tilejson_builder.name("tileset name");
    tilejson_builder.description("some description");

    let tiles = vec!["http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"];
    tilejson_builder.tiles(tiles);

    let tilejson = tilejson_builder.finalize();
    let serialized_tilejson = serde_json::to_string(&tilejson).unwrap();

    println!("{}", serialized_tilejson);
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
