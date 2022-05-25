//! # TileJSON
//!
//! `tilejson` is a crate for serializing/deserializing
//! [TileJSON format](https://github.com/mapbox/tilejson-spec) —
//! an open standard for representing map metadata.
//!
//! Use [tilejson!] macro to instantiate a valid [TileJSON].
//! Use [TileJSON::set_missing_defaults] to populate default values per spec.

mod bounds;
mod center;
mod tilejson;
mod vector_layer;

pub use crate::bounds::*;
pub use crate::center::*;
pub use crate::tilejson::*;
pub use crate::vector_layer::*;
