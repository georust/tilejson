#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct TileJSON {
  tilejson: &'static str,
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
  center: Option<Vec<i32>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileJSONBuilder {
  tilejson: &'static str,
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
  center: Option<Vec<i32>>
}

impl TileJSONBuilder {
  pub fn new() -> TileJSONBuilder {
    TileJSONBuilder {
      tilejson: "2.2.0",
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
      center: None
    }
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

  pub fn tiles(&mut self, tiles: Vec<String>) -> &mut TileJSONBuilder {
    self.tiles = tiles;
    self
  }

  pub fn grids(&mut self, grids: Vec<String>) -> &mut TileJSONBuilder {
    self.grids = Some(grids);
    self
  }

  pub fn data(&mut self, data: Vec<String>) -> &mut TileJSONBuilder {
    self.data = Some(data);
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
