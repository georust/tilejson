use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug, Copy, Clone)]
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
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds>
    fn default() -> Self {
        Self::new(-180.0, -85.05112877980659, 180.0, 85.0511287798066)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseBoundsError {
    /// Incorrect number of values
    BadLen,
    /// Wrapped error from the parse::<f64>()
    ParseCoordError(ParseFloatError),
}

impl Display for ParseBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseBoundsError::BadLen => {
                f.write_str("Incorrect number of values. Bounds expects four f64 values.")
            }
            ParseBoundsError::ParseCoordError(e) => e.fmt(f),
        }
    }
}

impl TryFrom<Vec<f64>> for Bounds {
    type Error = ParseBoundsError;

    /// Parse four f64 values as a Bounds value, same order as the [Bounds::new] constructor.
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            Ok(Self {
                left: value[0],
                bottom: value[1],
                right: value[2],
                top: value[3],
            })
        } else {
            Err(ParseBoundsError::BadLen)
        }
    }
}

impl FromStr for Bounds {
    type Err = ParseBoundsError;

    /// Parse a string of four comma-separated values as a Bounds value,
    /// same order as the [Bounds::new] constructor. Extra spaces are ignored.
    ///
    /// # Example
    /// ```
    /// # use tilejson::Bounds;
    /// # use std::str::FromStr;
    /// let bounds = Bounds::from_str("-1.0, -2.0, 3, 4").unwrap();
    /// assert_eq!(bounds, Bounds::new(-1.0, -2.0, 3.0, 4.0));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',').map(|s| s.trim());
        let mut next_val = || {
            vals.next().map_or(Err(ParseBoundsError::BadLen), |v| {
                v.parse().map_err(ParseBoundsError::ParseCoordError)
            })
        };
        let bounds = Self {
            left: next_val()?,
            bottom: next_val()?,
            right: next_val()?,
            top: next_val()?,
        };
        match vals.next() {
            Some(_) => Err(ParseBoundsError::BadLen),
            None => Ok(bounds),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_err() {
        const E_EMPTY: &str = "cannot parse float from empty string";
        const E_FORMAT: &str = "invalid float literal";
        const E_LEN: &str = "Incorrect number of values. Bounds expects four f64 values.";

        let err_to_str = |v| Bounds::from_str(v).unwrap_err().to_string();

        assert_eq!(err_to_str(""), E_EMPTY);
        assert_eq!(err_to_str("1"), E_LEN);
        assert_eq!(err_to_str("1,2,3"), E_LEN);
        assert_eq!(err_to_str("1,2,3,4,5"), E_LEN);
        assert_eq!(err_to_str("1,2,3,a"), E_FORMAT);
    }

    #[test]
    fn test_parse() {
        let val = |s| Bounds::from_str(s).unwrap();
        assert_eq!(val("0,0,0,0"), Bounds::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(val(" 1 ,2.0, 3.0,  4.0 "), Bounds::new(1.0, 2.0, 3.0, 4.0));
    }
}
