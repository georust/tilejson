use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::collections::HashMap;

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug, Default)]
pub struct Center {
    pub longitude: f64,
    pub latitude: f64,
    pub zoom: u8,
}

impl Center {
    pub fn new(longitude: f64, latitude: f64, zoom: u8) -> Self {
        Self {
            longitude,
            latitude,
            zoom,
        }
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug)]
pub struct Bounds {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Bounds {
    pub fn new(left: f64, bottom: f64, right: f64, top: f64) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }
}

impl Default for Bounds {
    /// Default bounds are set to `[-180, -85.05112877980659, 180, 85.0511287798066]`
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds
    fn default() -> Self {
        Self::new(-180.0, -85.05112877980659, 180.0, 85.0511287798066)
    }
}

impl From<Vec<f64>> for Bounds {
    fn from(item: Vec<f64>) -> Self {
        assert_eq!(item.len(), 4, "bounds must be an array with 4 values");
        Self {
            left: item[0],
            bottom: item[1],
            right: item[2],
            top: item[3],
        }
    }
}

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
/// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#33-vector_layers
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct VectorLayer {
    /// A string value representing the the layer id.
    ///
    /// For added context, this is referred to as the name of the layer in the
    /// [Mapbox Vector Tile spec](https://github.com/mapbox/vector-tile-spec/tree/master/2.1#41-layers).
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#331-id
    pub id: String,

    /// An object whose keys and values are the names and descriptions of attributes available in this layer.
    ///
    /// Each value (description) MUST be a string that describes the underlying data.
    /// If no fields are present, the fields key MUST be an empty object.
    /// https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#332-fields
    pub fields: HashMap<String, String>,

    /// A string representing a human-readable description of the entire layer's contents.
    ///
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#333-description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An integer representing the highest zoom level whose tiles this layer appears in.
    ///
    /// maxzoom MUST be less than or equal to the set of tiles' maxzoom.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#334-minzoom-and-maxzoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,

    /// An integer representing the lowest zoom level whose tiles this layer appears in.
    ///
    /// minzoom MUST be greater than or equal to the set of tiles' minzoom.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#334-minzoom-and-maxzoom
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

/// TileJSON struct represents tilejson-spec metadata as specified by
/// https://github.com/mapbox/tilejson-spec (version 3.0.0)
/// Some descriptions were copied verbatim from the spec per CC-BY 3.0 license.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TileJSON {
    /// A semver.org style version number as a string.
    /// Describes the version of the TileJSON spec that is implemented by this JSON object.
    /// Example: `"3.0.0"`
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#31-tilejson
    pub tilejson: String,

    /// An array of tile endpoints.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination
    /// of endpoints. All endpoint urls MUST be absolute. All endpoints MUST return
    /// the same content for the same URL. The array MUST contain at least one endpoint.
    /// The tile extension is NOT limited to any particular format.
    /// Some of the more popular are: mvt, vector.pbf, png, webp, and jpg.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#32-tiles
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
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#33-vector_layers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_layers: Option<Vec<VectorLayer>>,

    /// Contains an attribution to be displayed when the map is shown to a user.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this content
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#34-attribution
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
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds
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
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#36-center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,

    /// An array of data files in GeoJSON format.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL. If the array doesn't
    /// contain any entries, then no data is present in the map. This field is for overlaying
    /// GeoJSON data on tiled raster maps and is generally no longer used for GL-based maps.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#37-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,

    /// A text description of the set of tiles.
    ///
    /// The description can contain any valid unicode character as described by the JSON specification RFC 8259.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#38-description
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
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#39-fillzoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fillzoom: Option<u8>,

    /// An array of interactivity endpoints.
    ///
    /// {z}, {x} and {y}, if present, are replaced with the corresponding integers.
    /// If multiple endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL. If the array doesn't
    /// contain any entries, UTF-Grid interactivity is not supported for this set of tiles.
    /// See https://github.com/mapbox/utfgrid-spec/tree/master/1.2 for the interactivity specification.
    ///
    /// *Note: UTF-Grid interactivity predates GL-based map rendering and interaction.
    /// Map interactivity is now generally defined outside of the TileJSON specification
    /// and is dependent on the tile rendering library's features.*
    ///
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#310-grids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grids: Option<Vec<String>>,

    /// Contains a legend to be displayed with the map.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this field
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#311-legend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,

    /// An integer specifying the maximum zoom level.
    ///
    /// MUST be in range: 0 <= minzoom <= maxzoom <= 30.
    /// A client or server MAY request tiles outside of the zoom range,
    /// but the availability of these tiles is dependent on how the the tile server
    /// or renderer handles the request (such as overzooming tiles).
    /// OPTIONAL. Integer. Default: 30.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#312-maxzoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,

    /// An integer specifying the minimum zoom level.
    ///
    /// MUST be in range: 0 <= minzoom <= maxzoom <= 30.
    /// OPTIONAL. Integer. Default: 0.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#313-minzoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,

    /// A name describing the tileset.
    ///
    /// The name can contain any legal character.
    /// Implementations SHOULD NOT interpret the name as HTML.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#314-name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Either "xyz" or "tms".
    ///
    /// Influences the y direction of the tile coordinates.
    /// The global-mercator (aka Spherical Mercator) profile is assumed.
    /// OPTIONAL. String. Default: "xyz".
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#315-scheme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// Contains a mustache template to be used to format data from grids for interaction.
    ///
    /// See https://github.com/mapbox/utfgrid-spec/tree/master/1.2 for the interactivity specification.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#316-template
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
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#317-version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Any unrecognized fields will be stored here
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl TileJSON {
    /// create a builder with tilejson = 3.0.0 and tiles = `[ source ]`
    pub fn new(source: String) -> Self {
        Self::new_ext(vec![source], None)
    }

    /// create a builder with a custom version and multiple sources.
    /// If version is None, use the current default.
    pub fn new_ext(tiles_sources: Vec<String>, tilejson_version: Option<String>) -> Self {
        Self {
            tilejson: tilejson_version.unwrap_or_else(|| "3.0.0".to_string()),
            tiles: tiles_sources,
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

    /// Set any missing default values per tile-json specification
    pub fn set_missing_defaults(&mut self) {
        self.version.get_or_insert_with(|| "1.0.0".to_string());
        self.scheme.get_or_insert_with(|| "xyz".to_string());
        self.minzoom.get_or_insert(0);
        self.maxzoom.get_or_insert(30);
        self.bounds.get_or_insert_with(Bounds::default);
    }

    pub fn vector_layers(mut self, vector_layers: Vec<VectorLayer>) -> Self {
        self.vector_layers = Some(vector_layers);
        self
    }

    pub fn attribution(mut self, attribution: String) -> Self {
        self.attribution = Some(attribution);
        self
    }

    pub fn bounds(mut self, bounds: Bounds) -> Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn center(mut self, center: Center) -> Self {
        self.center = Some(center);
        self
    }

    pub fn data(mut self, data: Vec<String>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn fillzoom(mut self, fillzoom: u8) -> Self {
        self.fillzoom = Some(fillzoom);
        self
    }

    pub fn grids(mut self, grids: Vec<String>) -> Self {
        self.grids = Some(grids);
        self
    }

    pub fn legend(mut self, legend: String) -> Self {
        self.legend = Some(legend);
        self
    }

    pub fn maxzoom(mut self, maxzoom: u8) -> Self {
        self.maxzoom = Some(maxzoom);
        self
    }

    pub fn minzoom(mut self, minzoom: u8) -> Self {
        self.minzoom = Some(minzoom);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn scheme(mut self, scheme: String) -> Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }
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
            TileJSON {
                tilejson: "3.0.0".to_string(),
                tiles: vec!["http://localhost:8888/foo/{z}/{x}/{y}.png".to_string()],
                vector_layers: None,
                attribution: Some("".to_string()),
                bounds: None,
                center: None,
                data: None,
                description: None,
                fillzoom: None,
                grids: None,
                legend: None,
                maxzoom: None,
                minzoom: None,
                name: Some("compositing".to_string()),
                scheme: Some("tms".to_string()),
                template: None,
                version: None,
                other: Default::default()
            }
        );

        tilejson.set_missing_defaults();

        assert_eq!(
            tilejson,
            TileJSON {
                tilejson: "3.0.0".to_string(),
                tiles: vec!["http://localhost:8888/foo/{z}/{x}/{y}.png".to_string()],
                vector_layers: None,
                attribution: Some("".to_string()),
                bounds: Some(Bounds::new(
                    -180.0,
                    -85.05112877980659,
                    180.0,
                    85.0511287798066,
                )),
                center: None,
                data: None,
                description: None,
                fillzoom: None,
                grids: None,
                legend: None,
                maxzoom: Some(30),
                minzoom: Some(0),
                name: Some("compositing".to_string()),
                scheme: Some("tms".to_string()),
                template: None,
                version: Some("1.0.0".to_string()),
                other: Default::default()
            }
        );
    }

    #[test]
    fn test_writing() {
        let source = "http://localhost:8888/foo/{z}/{x}/{y}.png";
        let tilejson = TileJSON::new(source.to_string())
            .name("compositing".to_string())
            .scheme("tms".to_string())
            .bounds(Bounds::new(-1.0, -2.0, 3.0, 4.0))
            .center(Center::new(-5.0, -6.0, 3));

        assert_eq!(
            serde_json::to_string(&tilejson).unwrap(),
            r#"{"tilejson":"3.0.0","tiles":["http://localhost:8888/foo/{z}/{x}/{y}.png"],"bounds":[-1.0,-2.0,3.0,4.0],"center":[-5.0,-6.0,3],"name":"compositing","scheme":"tms"}"#,
        );

        let tilejson = TileJSON::new(source.to_string()).vector_layers(vec![VectorLayer::new(
            "a".to_string(),
            HashMap::from([("b".to_string(), "c".to_string())]),
        )]);
        assert_eq!(
            serde_json::to_string(&tilejson).unwrap(),
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
