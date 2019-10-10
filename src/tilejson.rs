extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

/// TileJSON struct that represents map metadata
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TileJSON {
  /// A semver.org style version number. Describes the version of
  /// the TileJSON spec that is implemented by this JSON object.
  pub tilejson: &'static str,

  /// The tileset id.
  pub id: Option<String>,

  /// A name describing the tileset. The name can
  /// contain any legal character. Implementations SHOULD NOT interpret the
  /// name as HTML.
  pub name: Option<String>,

  /// A text description of the tileset. The
  /// description can contain any legal character. Implementations SHOULD NOT
  /// interpret the description as HTML.
  pub description: Option<String>,

  /// A semver.org style version number. When
  /// changes across tiles are introduced, the minor version MUST change.
  /// This may lead to cut off labels. Therefore, implementors can decide to
  /// clean their cache when the minor version changes. Changes to the patch
  /// level MUST only have changes to tiles that are contained within one tile.
  /// When tiles change significantly, the major version MUST be increased.
  /// Implementations MUST NOT use tiles with different major versions.
  pub version: Option<String>,

  /// Contains an attribution to be displayed
  /// when the map is shown to a user. Implementations MAY decide to treat this
  /// as HTML or literal text. For security reasons, make absolutely sure that
  /// this field can't be abused as a vector for XSS or beacon tracking.
  pub attribution: Option<String>,

  /// Contains a mustache template to be used to
  /// format data from grids for interaction.
  /// See https://github.com/mapbox/utfgrid-spec/tree/master/1.2
  /// for the interactivity specification.
  pub template: Option<String>,

  /// Contains a legend to be displayed with the map.
  /// Implementations MAY decide to treat this as HTML or literal text.
  /// For security reasons, make absolutely sure that this field can't be
  /// abused as a vector for XSS or beacon tracking.
  pub legend: Option<String>,

  /// Either "xyz" or "tms". Influences the y
  /// direction of the tile coordinates.
  /// The global-mercator (aka Spherical Mercator) profile is assumed.
  pub scheme: Option<String>,

  /// An array of tile endpoints. {z}, {x} and {y}, if present,
  /// are replaced with the corresponding integers. If multiple endpoints are specified, clients
  /// may use any combination of endpoints. All endpoints MUST return the same
  /// content for the same URL. The array MUST contain at least one endpoint.
  pub tiles: Vec<String>,

  /// An array of interactivity endpoints. {z}, {x}
  /// and {y}, if present, are replaced with the corresponding integers. If multiple
  /// endpoints are specified, clients may use any combination of endpoints.
  /// All endpoints MUST return the same content for the same URL.
  /// If the array doesn't contain any entries, interactivity is not supported
  /// for this tileset.
  /// See https://github.com/mapbox/utfgrid-spec/tree/master/1.2
  /// for the interactivity specification.
  pub grids: Option<Vec<String>>,

  /// An array of data files in GeoJSON format.
  /// {z}, {x} and {y}, if present,
  /// are replaced with the corresponding integers. If multiple
  /// endpoints are specified, clients may use any combination of endpoints.
  /// All endpoints MUST return the same content for the same URL.
  /// If the array doesn't contain any entries, then no data is present in
  /// the map.
  pub data: Option<Vec<String>>,

  /// An integer specifying the minimum zoom level.
  pub minzoom: Option<u8>,

  /// An integer specifying the maximum zoom level. MUST be >= minzoom.
  pub maxzoom: Option<u8>,

  /// The maximum extent of available map tiles. Bounds MUST define an area
  /// covered by all zoom levels. The bounds are represented in WGS:84
  /// latitude and longitude values, in the order left, bottom, right, top.
  /// Values may be integers or floating point numbers.
  pub bounds: Option<Vec<i32>>,

  /// The first value is the longitude, the second is latitude (both in
  /// WGS:84 values), the third value is the zoom level as an integer.
  /// Longitude and latitude MUST be within the specified bounds.
  /// The zoom level MUST be between minzoom and maxzoom.
  /// Implementations can use this value to set the default location. If the
  /// value is null, implementations may use their own algorithm for
  /// determining a default location.
  pub center: Option<Vec<i32>>,
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
  grids: Option<Vec<String>>,
  data: Option<Vec<String>>,
  minzoom: Option<u8>,
  maxzoom: Option<u8>,
  bounds: Option<Vec<i32>>,
  center: Option<Vec<i32>>,
}

impl TileJSONBuilder {
  pub fn new() -> TileJSONBuilder {
    TileJSONBuilder {
      tilejson: "2.2.0",
      id: None,
      name: None,
      description: None,
      version: Some("1.0.0".to_owned()),
      attribution: None,
      template: None,
      legend: None,
      scheme: Some("xyz".to_owned()),
      tiles: Vec::new(),
      grids: None,
      data: None,
      minzoom: Some(0),
      maxzoom: Some(30),
      bounds: Some(vec![-180, -90, 180, 90]),
      center: None,
    }
  }

  pub fn id(&mut self, id: &str) -> &mut TileJSONBuilder {
    self.id = Some(id.to_string());
    self
  }

  pub fn name(&mut self, name: &str) -> &mut TileJSONBuilder {
    self.name = Some(name.to_string());
    self
  }

  pub fn description(&mut self, description: &str) -> &mut TileJSONBuilder {
    self.description = Some(description.to_string());
    self
  }

  pub fn version(&mut self, version: &str) -> &mut TileJSONBuilder {
    self.version = Some(version.to_string());
    self
  }

  pub fn attribution(&mut self, attribution: &str) -> &mut TileJSONBuilder {
    self.attribution = Some(attribution.to_string());
    self
  }

  pub fn template(&mut self, template: &str) -> &mut TileJSONBuilder {
    self.template = Some(template.to_string());
    self
  }

  pub fn legend(&mut self, legend: &str) -> &mut TileJSONBuilder {
    self.legend = Some(legend.to_string());
    self
  }

  pub fn scheme(&mut self, scheme: &str) -> &mut TileJSONBuilder {
    self.scheme = Some(scheme.to_string());
    self
  }

  pub fn tiles(&mut self, tiles: Vec<&str>) -> &mut TileJSONBuilder {
    self.tiles = tiles.into_iter().map(|url| url.to_owned()).collect();
    self
  }

  pub fn grids(&mut self, grids: Vec<&str>) -> &mut TileJSONBuilder {
    self.grids = Some(grids.into_iter().map(|url| url.to_owned()).collect());
    self
  }

  pub fn data(&mut self, data: Vec<&str>) -> &mut TileJSONBuilder {
    self.data = Some(data.into_iter().map(|url| url.to_owned()).collect());
    self
  }

  pub fn minzoom(&mut self, minzoom: u8) -> &mut TileJSONBuilder {
    self.minzoom = Some(minzoom);
    self
  }

  pub fn maxzoom(&mut self, maxzoom: u8) -> &mut TileJSONBuilder {
    self.maxzoom = Some(maxzoom);
    self
  }

  pub fn bounds(&mut self, bounds: Vec<i32>) -> &mut TileJSONBuilder {
    self.bounds = Some(bounds);
    self
  }

  pub fn center(&mut self, center: Vec<i32>) -> &mut TileJSONBuilder {
    self.center = Some(center);
    self
  }

  pub fn finalize(self) -> TileJSON {
    TileJSON {
      tilejson: self.tilejson,
      id: self.id,
      name: self.name,
      description: self.description,
      version: self.version,
      attribution: self.attribution,
      template: self.template,
      legend: self.legend,
      scheme: self.scheme,
      tiles: self.tiles,
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
        "tilejson": "2.2.0",
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
        tilejson: "2.2.0",
        id: None,
        name: Some(String::from("compositing")),
        description: None,
        version: None,
        attribution: None,
        template: None,
        legend: None,
        scheme: Some(String::from("tms")),
        tiles: vec![String::from(
          "http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"
        )],
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
    let mut tilejson_builder = TileJSONBuilder::new();

    tilejson_builder.name("compositing");
    tilejson_builder.scheme("tms");

    let tiles = vec!["http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"];
    tilejson_builder.tiles(tiles);

    let tilejson = tilejson_builder.finalize();
    let serialized_tilejson = serde_json::to_string(&tilejson).unwrap();

    assert_eq!(
      serialized_tilejson,
      r#"{"tilejson":"2.2.0","id":null,"name":"compositing","description":null,"version":"1.0.0","attribution":null,"template":null,"legend":null,"scheme":"tms","tiles":["http://localhost:8888/admin/1.0.0/world-light,broadband/{z}/{x}/{y}.png"],"grids":null,"data":null,"minzoom":0,"maxzoom":30,"bounds":[-180,-90,180,90],"center":null}"#
    )
  }
}
