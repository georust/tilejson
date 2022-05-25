use crate::bounds::Bounds;
use crate::center::Center;
use crate::vector_layer::VectorLayer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// TileJSON struct represents tilejson-spec metadata as specified by
/// <https://github.com/mapbox/tilejson-spec> (version 3.0.0)
/// Some descriptions were copied verbatim from the spec per CC-BY 3.0 license.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TileJSON {
    /// A semver.org style version number as a string.
    /// Describes the version of the TileJSON spec that is implemented by this JSON object.
    /// Example: `"3.0.0"`
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#31-tilejson>
    pub tilejson: String,

    /// An array of tile endpoints.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination
    /// of endpoints. All endpoint urls MUST be absolute. All endpoints MUST return
    /// the same content for the same URL. The array MUST contain at least one endpoint.
    /// The tile extension is NOT limited to any particular format.
    /// Some of the more popular are: mvt, vector.pbf, png, webp, and jpg.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#32-tiles>
    pub tiles: Vec<String>,

    /// An array of objects. Each object describes one layer of vector tile data.
    ///
    /// A vector_layer object MUST contain the id and fields keys, and MAY contain the description,
    /// minzoom, or maxzoom keys. An implementation MAY include arbitrary keys in the object
    /// outside of those defined in this specification.
    ///
    /// *Note: When describing a set of raster tiles or other tile format that does not have
    /// a "layers" concept (i.e. "format": "jpeg"), the vector_layers key is not required.*
    ///
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#33-vector_layers>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_layers: Option<Vec<VectorLayer>>,

    /// Contains an attribution to be displayed when the map is shown to a user.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this content
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#34-attribution>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// The maximum extent of available map tiles.
    ///
    /// Bounds MUST define an area covered by all zoom levels. The bounds are represented
    /// in WGS 84 latitude and longitude values, in the order left, bottom, right, top.
    /// Values may be integers or floating point numbers. The minimum/maximum values for
    /// longitude and latitude are -180/180 and -90/90 respectively. Bounds MUST NOT "wrap"
    /// around the ante-meridian. If bounds are not present, the default value MAY assume
    /// the set of tiles is globally distributed.
    ///
    /// Bounds where longitude values are the same, and latitude values are the same,
    /// are considered valid. This case typically represents a single point geometry
    /// in the entire tileset. For example: `[-122.34, 47.65, -122.34, 47.65]`.
    ///
    /// OPTIONAL. Array. Default: `[ -180, -85.05112877980659, 180, 85.0511287798066 ]` (xyz-compliant tile bounds)
    ///
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,

    /// The first value is the longitude, the second is latitude (both in WGS:84 values),
    ///
    /// the third value is the zoom level as an integer. Longitude and latitude MUST be
    /// within the specified bounds. The zoom level MUST be between minzoom and maxzoom.
    /// Implementations MAY use this center value to set the default location. If the value
    /// is null, implementations MAY use their own algorithm for determining a default location.\
    /// OPTIONAL. Array. Default: null.
    /// Example: `"center": [ -76.275329586789, 39.153492567373, 8 ]`
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#36-center>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,

    /// An array of data files in GeoJSON format.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL. If the array doesn't
    /// contain any entries, then no data is present in the map. This field is for overlaying
    /// GeoJSON data on tiled raster maps and is generally no longer used for GL-based maps.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#37-data>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,

    /// A text description of the set of tiles.
    ///
    /// The description can contain any valid unicode character as described by the JSON specification RFC 8259.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#38-description>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An integer specifying the zoom level from which to generate overzoomed tiles.
    ///
    /// Implementations MAY generate overzoomed tiles from parent tiles if the requested
    /// zoom level does not exist. In most cases, overzoomed tiles are generated from
    /// the maximum zoom level of the set of tiles. If fillzoom is specified, the overzoomed
    /// tile MAY be generated from the fillzoom level.
    ///
    /// For example, in a set of tiles with maxzoom 10 and no fillzoom specified,
    /// a request for a z11 tile will use the z10 parent tiles to generate the new,
    /// overzoomed z11 tile. If the same TileJSON object had fillzoom specified at z7,
    /// a request for a z11 tile would use the z7 tile instead of z10.
    ///
    /// While TileJSON may specify rules for overzooming tiles, it is ultimately up to the tile
    /// serving client or renderer to implement overzooming.
    ///
    /// OPTIONAL. Integer. Default: null.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#39-fillzoom>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fillzoom: Option<u8>,

    /// An array of interactivity endpoints.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL. If the array doesn't
    /// contain any entries, UTF-Grid interactivity is not supported for this set of tiles.
    /// See <https://github.com/mapbox/utfgrid-spec/tree/master/1.2> for the interactivity specification.
    ///
    /// *Note: UTF-Grid interactivity predates GL-based map rendering and interaction.
    /// Map interactivity is now generally defined outside of the TileJSON specification
    /// and is dependent on the tile rendering library's features.*
    ///
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#310-grids>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grids: Option<Vec<String>>,

    /// Contains a legend to be displayed with the map.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this field
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#311-legend>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,

    /// An integer specifying the maximum zoom level.
    ///
    /// MUST be in range: 0 <= minzoom <= maxzoom <= 30.
    /// A client or server MAY request tiles outside of the zoom range,
    /// but the availability of these tiles is dependent on how the the tile server
    /// or renderer handles the request (such as overzooming tiles).
    /// OPTIONAL. Integer. Default: 30.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#312-maxzoom>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,

    /// An integer specifying the minimum zoom level.
    ///
    /// MUST be in range: 0 <= minzoom <= maxzoom <= 30.
    /// OPTIONAL. Integer. Default: 0.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#313-minzoom>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,

    /// A name describing the tileset.
    ///
    /// The name can contain any legal character.
    /// Implementations SHOULD NOT interpret the name as HTML.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#314-name>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Either "xyz" or "tms".
    ///
    /// Influences the y direction of the tile coordinates.
    /// The global-mercator (aka Spherical Mercator) profile is assumed.
    /// OPTIONAL. String. Default: "xyz".
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#315-scheme>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// Contains a mustache template to be used to format data from grids for interaction.
    ///
    /// See <https://github.com/mapbox/utfgrid-spec/tree/master/1.2> for the interactivity specification.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#316-template>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// A semver.org style version number of the tiles.
    ///
    /// When changes across tiles are introduced the minor version MUST change.
    /// This may lead to cut off labels. Therefore, implementors can decide to clean
    /// their cache when the minor version changes. Changes to the patch level MUST
    /// only have changes to tiles that are contained within one tile.
    /// When tiles change significantly, such as updating a vector tile layer name,
    /// the major version MUST be increased.
    /// Implementations MUST NOT use tiles with different major versions.
    /// OPTIONAL. String. Default: "1.0.0".
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#317-version>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Any unrecognized fields will be stored here
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl TileJSON {
    /// Set any missing default values per tile-json specification
    pub fn set_missing_defaults(&mut self) {
        self.version.get_or_insert_with(|| "1.0.0".to_string());
        self.scheme.get_or_insert_with(|| "xyz".to_string());
        self.minzoom.get_or_insert(0);
        self.maxzoom.get_or_insert(30);
        self.bounds.get_or_insert_with(Bounds::default);
    }
}

/// Use this macro to create a TileJSON struct with optional values.
/// The `tilejson!` macro can be used in several ways:
///
/// ### With a single tile source
/// ```
/// # use crate::tilejson::tilejson;
/// // The tile source is auto-converted to a vector
/// let tj = tilejson! { "https://example.com/".to_string() };
/// assert_eq!(tj.tiles[0], "https://example.com/");
/// assert_eq!(tj.tilejson, "3.0.0");
/// assert_eq!(tj.minzoom, None);
///
/// // With optional values
/// let tj = tilejson! { "https://example.com/".to_string(), minzoom: 1, maxzoom: 2 };
/// assert_eq!(tj.tiles[0], "https://example.com/");
/// assert_eq!(tj.minzoom, Some(1));
/// assert_eq!(tj.maxzoom, Some(2));
/// ```
///
/// ### With multiple tile sources and an optional version
/// ```
/// # use crate::tilejson::tilejson;
/// // Could use any number of tile sources here
/// let tj = tilejson! { tiles: vec!["https://example.com/".to_string()] };
/// assert_eq!(tj.tiles[0], "https://example.com/");
///
/// // With the optional tilejson version (must be used in this order)
/// let tj = tilejson! { "https://example.com/".to_string(), tilejson: "2.1.0".to_string() };
/// assert_eq!(tj.tiles[0], "https://example.com/");
/// assert_eq!(tj.tilejson, "2.1.0");
///
/// // Other optional values could be used at the end
/// let tj = tilejson! { "https://example.com/".to_string(), minzoom: 5 };
/// assert_eq!(tj.tiles[0], "https://example.com/");
/// assert_eq!(tj.tilejson, "3.0.0");
/// assert_eq!(tj.minzoom, Some(5));
///
/// // version and optional values together
/// let tj = tilejson! { "https://example.com/".to_string(), tilejson: "2.2.0".to_string(), minzoom: 5 };
/// assert_eq!(tj.tiles[0], "https://example.com/");
/// assert_eq!(tj.tilejson, "2.2.0");
/// assert_eq!(tj.minzoom, Some(5));
/// ```
#[macro_export]
macro_rules! tilejson {
    ( tiles: $sources:expr, tilejson: $ver:expr $(, $tag:tt : $val:expr)* $(,)? ) => {
        $crate::TileJSON {
            $( $tag: Some($val), )*
            ..$crate::TileJSON {
                tilejson: $ver,
                tiles: $sources,
                vector_layers: None,
                attribution: None,
                bounds: None,
                center: None,
                data: None,
                description: None,
                fillzoom: None,
                grids: None,
                legend: None,
                maxzoom: None,
                minzoom: None,
                name: None,
                scheme: None,
                template: None,
                version: None,
                other: Default::default(),
            }
        }
    };
    ( tiles: $sources:expr $(, $tag:tt : $val:expr)* $(,)? ) => {
        $crate::tilejson! {
            tiles: $sources,
            tilejson: "3.0.0".to_string(),
            $( $tag: $val , )* }
    };
    ( $tile_source:expr $(, $tag:tt : $val:expr)* $(,)? ) => {
        $crate::tilejson! {
            tiles: vec! [ $tile_source ],
            $( $tag: $val , )* }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let tilejson_str = r#"{
        "tilejson": "3.0.0",
        "attribution": "",
        "name": "compositing",
        "scheme": "tms",
        "tiles": [
            "http://localhost:8888/foo/{z}/{x}/{y}.png"
        ]
    }"#;

        let mut tilejson: TileJSON = serde_json::from_str(&tilejson_str).unwrap();

        assert_eq!(
            tilejson,
            tilejson! {
                tiles: vec!["http://localhost:8888/foo/{z}/{x}/{y}.png".to_string()],
                tilejson: "3.0.0".to_string(),
                attribution: "".to_string(),
                name: "compositing".to_string(),
                scheme: "tms".to_string(),
            }
        );

        tilejson.set_missing_defaults();

        assert_eq!(
            tilejson,
            tilejson! {
                tiles: vec!["http://localhost:8888/foo/{z}/{x}/{y}.png".to_string()],
                tilejson: "3.0.0".to_string(),
                attribution: "".to_string(),
                name: "compositing".to_string(),
                scheme: "tms".to_string(),
                bounds: Bounds::new(
                    -180.0,
                    -85.05112877980659,
                    180.0,
                    85.0511287798066,
                ),
                maxzoom: 30,
                minzoom: 0,
                version: "1.0.0".to_string(),
            }
        );
    }

    #[test]
    fn test_writing() {
        let source = "http://localhost:8888/foo/{z}/{x}/{y}.png";
        let tj = tilejson! {
            source.to_string(),
            name: "compositing".to_string(),
            scheme: "tms".to_string(),
            bounds: Bounds::new(-1.0, -2.0, 3.0, 4.0),
            center: Center::new(-5.0, -6.0, 3),
        };

        assert_eq!(
            serde_json::to_string(&tj).unwrap(),
            r#"{"tilejson":"3.0.0","tiles":["http://localhost:8888/foo/{z}/{x}/{y}.png"],"bounds":[-1.0,-2.0,3.0,4.0],"center":[-5.0,-6.0,3],"name":"compositing","scheme":"tms"}"#,
        );

        let vl = VectorLayer::new(
            "a".to_string(),
            HashMap::from([("b".to_string(), "c".to_string())]),
        );
        let tj = tilejson! {
            source.to_string(),
            vector_layers: vec![vl]
        };

        assert_eq!(
            serde_json::to_string(&tj).unwrap(),
            r#"{"tilejson":"3.0.0","tiles":["http://localhost:8888/foo/{z}/{x}/{y}.png"],"vector_layers":[{"id":"a","fields":{"b":"c"}}]}"#,
        );
    }

    fn parse(json_str: &str) -> serde_json::Result<TileJSON> {
        serde_json::from_str(json_str)
    }

    #[test]
    fn test_bad_json() {
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "center":[]}"#).unwrap_err();
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "center":[1,2]}"#).unwrap_err();
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "center":[1,2,3,4]}"#).unwrap_err();
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "bounds":[]}"#).unwrap_err();
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "bounds":[1,2,3]}"#).unwrap_err();
        parse(&r#"{"tilejson":"3.0.0", "tiles":["x"], "bounds":[1,2,3,4,5]}"#).unwrap_err();
    }
}
