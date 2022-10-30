use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseCenterError {
    /// Incorrect number of values
    BadLen,
    /// Wrapped error from the parse::<f64>()
    ParseCoordError(ParseFloatError),
    /// Wrapped error from the parse::<u8>()
    ParseZoomError(ParseIntError),
}

impl Display for ParseCenterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCenterError::BadLen => {
                f.write_str("Incorrect number of values. Center expects two f64 and one u8 values.")
            }
            ParseCenterError::ParseCoordError(e) => e.fmt(f),
            ParseCenterError::ParseZoomError(e) => e.fmt(f),
        }
    }
}

impl FromStr for Center {
    type Err = ParseCenterError;

    /// Parse a string of four comma-separated values as a Center value,
    /// same order as the [Center::new] constructor. Extra spaces are ignored.
    ///
    /// # Example
    /// ```
    /// # use tilejson::Center;
    /// # use std::str::FromStr;
    /// let center = Center::from_str("1.0, 2.0, 3").unwrap();
    /// assert_eq!(center, Center::new(1.0, 2.0, 3));
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',').map(|s| s.trim());
        let mut next_val = || vals.next().ok_or(ParseCenterError::BadLen);
        let center = Self {
            longitude: next_val()?
                .parse()
                .map_err(ParseCenterError::ParseCoordError)?,
            latitude: next_val()?
                .parse()
                .map_err(ParseCenterError::ParseCoordError)?,
            zoom: next_val()?
                .parse()
                .map_err(ParseCenterError::ParseZoomError)?,
        };
        match vals.next() {
            Some(_) => Err(ParseCenterError::BadLen),
            None => Ok(center),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_err() {
        const E_EMPTY: &str = "cannot parse float from empty string";
        const E_FORMAT: &str = "invalid digit found in string";
        const E_LEN: &str = "Incorrect number of values. Center expects two f64 and one u8 values.";

        let err_to_str = |s| Center::from_str(s).unwrap_err().to_string();

        assert_eq!(err_to_str(""), E_EMPTY);
        assert_eq!(err_to_str("1"), E_LEN);
        assert_eq!(err_to_str("1,2"), E_LEN);
        assert_eq!(err_to_str("1,2,3,4"), E_LEN);
        assert_eq!(err_to_str("1,2,a"), E_FORMAT);
        assert_eq!(err_to_str("1,2,1.1"), E_FORMAT);
        assert_eq!(err_to_str("1,,0"), E_EMPTY);
    }

    #[test]
    fn test_parse() {
        let val = |s| Center::from_str(s).unwrap();
        assert_eq!(val("0,0,0"), Center::new(0.0, 0.0, 0));
        assert_eq!(val("  1 ,2.0, 3 "), Center::new(1.0, 2.0, 3));
    }
}
