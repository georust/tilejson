use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

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

    /// The tileset id.
    ///
    /// **This field is not part of the TileJSON spec and possibly may need to be deleted.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A name describing the tileset.
    ///
    /// The name can contain any legal character.
    /// Implementations SHOULD NOT interpret the name as HTML.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#314-name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A text description of the set of tiles.
    ///
    /// The description can contain any valid unicode character as described by the JSON specification RFC 8259.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#38-description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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

    /// Contains an attribution to be displayed when the map is shown to a user.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this content
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#34-attribution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// Contains a mustache template to be used to format data from grids for interaction.
    ///
    /// See https://github.com/mapbox/utfgrid-spec/tree/master/1.2 for the interactivity specification.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#316-template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// Contains a legend to be displayed with the map.
    ///
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this field
    /// can't be abused as a vector for XSS or beacon tracking.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#311-legend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,

    /// Either "xyz" or "tms".
    ///
    /// Influences the y direction of the tile coordinates.
    /// The global-mercator (aka Spherical Mercator) profile is assumed.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#315-scheme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

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

    /// An integer specifying the minimum zoom level.
    ///
    /// MUST be in range: 0 <= minzoom <= maxzoom <= 30.
    /// OPTIONAL. Integer. Default: 0.
    /// See https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#313-minzoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileJSONBuilder {
    tilejson: &'static str,
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    attribution: Option<String>,
    template: Option<String>,
    legend: Option<String>,
    scheme: Option<String>,
    tiles: Vec<String>,
    fillzoom: Option<u8>,
    grids: Option<Vec<String>>,
    data: Option<Vec<String>>,
    minzoom: Option<u8>,
    maxzoom: Option<u8>,
    bounds: Option<Bounds>,
    center: Option<Center>,
}

impl Default for TileJSONBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TileJSONBuilder {
    // Ignore the missing default because instantiation might need to be reworked.
    // See https://github.com/georust/tilejson/issues/10
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tilejson: "3.0.0",
            id: None,
            name: None,
            description: None,
            version: Some("1.0.0".to_string()),
            attribution: None,
            template: None,
            legend: None,
            scheme: Some("xyz".to_string()),
            tiles: Vec::new(),
            fillzoom: None,
            grids: None,
            data: None,
            minzoom: Some(0),
            maxzoom: Some(30),
            bounds: Some(Bounds::new(-180.0, -90.0, 180.0, 90.0)),
            center: None,
        }
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }

    pub fn attribution(&mut self, attribution: &str) -> &mut Self {
        self.attribution = Some(attribution.to_string());
        self
    }

    pub fn template(&mut self, template: &str) -> &mut Self {
        self.template = Some(template.to_string());
        self
    }

    pub fn legend(&mut self, legend: &str) -> &mut Self {
        self.legend = Some(legend.to_string());
        self
    }

    pub fn scheme(&mut self, scheme: &str) -> &mut Self {
        self.scheme = Some(scheme.to_string());
        self
    }

    pub fn tiles(&mut self, tiles: Vec<&str>) -> &mut Self {
        self.tiles = tiles.into_iter().map(|url| url.to_string()).collect();
        self
    }

    pub fn fillzoom(&mut self, fillzoom: u8) -> &mut TileJSONBuilder {
        self.fillzoom = Some(fillzoom);
        self
    }

    pub fn grids(&mut self, grids: Vec<&str>) -> &mut Self {
        self.grids = Some(grids.into_iter().map(|url| url.to_string()).collect());
        self
    }

    pub fn data(&mut self, data: Vec<&str>) -> &mut Self {
        self.data = Some(data.into_iter().map(|url| url.to_string()).collect());
        self
    }

    pub fn minzoom(&mut self, minzoom: u8) -> &mut Self {
        self.minzoom = Some(minzoom);
        self
    }

    pub fn maxzoom(&mut self, maxzoom: u8) -> &mut Self {
        self.maxzoom = Some(maxzoom);
        self
    }

    pub fn bounds(&mut self, bounds: Bounds) -> &mut Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn center(&mut self, center: Center) -> &mut Self {
        self.center = Some(center);
        self
    }

    pub fn finalize(self) -> TileJSON {
        TileJSON {
            tilejson: self.tilejson.to_string(),
            id: self.id,
            name: self.name,
            description: self.description,
            version: self.version,
            attribution: self.attribution,
            template: self.template,
            legend: self.legend,
            scheme: self.scheme,
            tiles: self.tiles,
            fillzoom: self.fillzoom,
            grids: self.grids,
            data: self.data,
            minzoom: self.minzoom,
            maxzoom: self.maxzoom,
            bounds: self.bounds,
            center: self.center,
        }
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
            "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"
        ]
    }"#;

        let tilejson: TileJSON = serde_json::from_str(&tilejson_str).unwrap();

        assert_eq!(
            tilejson,
            TileJSON {
                tilejson: "3.0.0".to_string(),
                id: None,
                name: Some("compositing".to_string()),
                description: None,
                version: None,
                attribution: Some("".to_string()),
                template: None,
                legend: None,
                scheme: Some("tms".to_string()),
                tiles: vec![
                    "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"
                        .to_string()
                ],
                fillzoom: None,
                grids: None,
                data: None,
                minzoom: None,
                maxzoom: None,
                bounds: None,
                center: None,
            }
        )
    }

    #[test]
    fn test_writing() {
        let mut tjb = TileJSONBuilder::new();

        tjb.name("compositing");
        tjb.scheme("tms");

        let tiles = vec!["http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"];
        tjb.tiles(tiles);

        tjb.bounds(Bounds::new(-1.0, -2.0, 3.0, 4.0));
        tjb.center(Center::new(-5.0, -6.0, 3));

        let tilejson = tjb.finalize();
        let serialized_tilejson = serde_json::to_string(&tilejson).unwrap();

        assert_eq!(
            serialized_tilejson,
            r#"{"tilejson":"3.0.0","name":"compositing","version":"1.0.0","scheme":"tms","tiles":["http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"],"minzoom":0,"maxzoom":30,"bounds":[-1.0,-2.0,3.0,4.0],"center":[-5.0,-6.0,3]}"#
        )
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
