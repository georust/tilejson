use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Each object describes one layer of vector tile data.
///
/// A vector_layer object MUST contain the id and fields keys, and MAY contain the description,
/// minzoom, or maxzoom keys. An implementation MAY include arbitrary keys in the object
/// outside of those defined in this specification.
///
/// *Note: When describing a set of raster tiles or other tile format that does not have
/// a "layers" concept (i.e. "format": "jpeg"), the vector_layers key is not required.*
///
/// These keys are used to describe the situation where different sets of vector layers
/// appear in different zoom levels of the same set of tiles, for example in a case where
/// a "minor roads" layer is only present at high zoom levels.
///
/// ```json
/// {
///   "vector_layers": [
///     {
///       "id": "roads",
///       "description": "Roads and their attributes",
///       "minzoom": 2,
///       "maxzoom": 16,
///       "fields": {
///         "type": "One of: trunk, primary, secondary",
///         "lanes": "Number",
///         "name": "String",
///         "sidewalks": "Boolean"
///       }
///     },
///     {
///       "id": "countries",
///       "description": "Admin 0 (country) boundaries",
///       "minzoom": 0,
///       "maxzoom": 16,
///       "fields": {
///         "iso": "ISO 3166-1 Alpha-2 code",
///         "name": "English name of the country",
///         "name_ar": "Arabic name of the country"
///       }
///     },
///     {
///       "id": "buildings",
///       "description": "A layer with an empty fields object",
///       "fields": {}
///     }
///   ]
/// }
/// ```
///
/// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#33-vector_layers>
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct VectorLayer {
    /// A string value representing the the layer id.
    ///
    /// For added context, this is referred to as the name of the layer in the
    /// [Mapbox Vector Tile spec](https://github.com/mapbox/vector-tile-spec/tree/master/2.1#41-layers).
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#331-id>
    pub id: String,

    /// An object whose keys and values are the names and descriptions of attributes available in this layer.
    ///
    /// Each value (description) MUST be a string that describes the underlying data.
    /// If no fields are present, the fields key MUST be an empty object.
    /// <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#332-fields>
    pub fields: HashMap<String, String>,

    /// A string representing a human-readable description of the entire layer's contents.
    ///
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#333-description>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An integer representing the highest zoom level whose tiles this layer appears in.
    ///
    /// maxzoom MUST be less than or equal to the set of tiles' maxzoom.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#334-minzoom-and-maxzoom>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,

    /// An integer representing the lowest zoom level whose tiles this layer appears in.
    ///
    /// minzoom MUST be greater than or equal to the set of tiles' minzoom.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#334-minzoom-and-maxzoom>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,

    /// Any unrecognized fields will be stored here.
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl VectorLayer {
    pub fn new(id: String, fields: HashMap<String, String>) -> Self {
        Self {
            id,
            fields,
            description: None,
            maxzoom: None,
            minzoom: None,
            other: Default::default(),
        }
    }
}
