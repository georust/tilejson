use std::fmt::{Display, Formatter, Write as _};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use thiserror::Error;

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug, Default, Copy, Clone)]
pub struct Center {
    pub longitude: f64,
    pub latitude: f64,
    pub zoom: u8,
}

impl Center {
    #[must_use]
    pub fn new(longitude: f64, latitude: f64, zoom: u8) -> Self {
        Self {
            longitude,
            latitude,
            zoom,
        }
    }
}

impl Display for Center {
    /// Format center struct as a comma-separated string.
    /// Longitude and latitude are formatted with specified precision parameters.
    ///
    /// ```
    /// # use tilejson::Center;
    /// # use std::str::FromStr;
    /// let center = Center::new(1.5, -2.5, 8);
    /// assert_eq!(center.to_string(), "1.5,-2.5,8");
    /// assert_eq!(format!("{center:.2}"), "1.50,-2.50,8");
    /// assert_eq!(Center::from_str(&center.to_string()).unwrap(), center);
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.longitude.fmt(f)?;
        f.write_char(',')?;
        self.latitude.fmt(f)?;
        f.write_char(',')?;
        write!(f, "{}", self.zoom)
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ParseCenterError {
    #[error("Incorrect number of values. Center expects two f64 and one u8 values.")]
    BadLen,
    /// Wrapped error from the `parse::<f64>()`
    #[error(transparent)]
    ParseCoordError(#[from] ParseFloatError),
    /// Wrapped error from the `parse::<u8>()`
    #[error(transparent)]
    ParseZoomError(#[from] ParseIntError),
}

impl From<(f64, f64, u8)> for Center {
    /// Parse a tuple as a Center value, same order as the [`Center::new`] constructor.
    ///
    /// ```
    /// # use tilejson::Center;
    /// assert_eq!(
    ///     Center::new(1.0, 2.0, 3),
    ///     Center::from((1.0_f64, 2.0_f64, 3))
    /// );
    /// ```
    fn from(value: (f64, f64, u8)) -> Self {
        Self {
            longitude: value.0,
            latitude: value.1,
            zoom: value.2,
        }
    }
}

impl From<(f32, f32, u8)> for Center {
    /// Parse a tuple as a Center value, same order as the [`Center::new`] constructor.
    ///
    /// ```
    /// # use tilejson::Center;
    /// assert_eq!(
    ///     Center::new(1.0, 2.0, 3),
    ///     Center::from((1.0_f32, 2.0_f32, 3))
    /// );
    /// ```
    fn from(value: (f32, f32, u8)) -> Self {
        Self {
            longitude: f64::from(value.0),
            latitude: f64::from(value.1),
            zoom: value.2,
        }
    }
}

impl FromStr for Center {
    type Err = ParseCenterError;

    /// Parse a string of four comma-separated values as a Center value,
    /// same order as the [`Center::new`] constructor. Extra spaces are ignored.
    ///
    /// # Example
    /// ```
    /// # use tilejson::Center;
    /// # use std::str::FromStr;
    /// let center = Center::from_str("1.0, 2.0, 3").unwrap();
    /// assert_eq!(center, Center::new(1.0, 2.0, 3));
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',').map(str::trim);
        let mut next_val = || vals.next().ok_or(ParseCenterError::BadLen);
        let center = Self {
            longitude: next_val()?.parse()?,
            latitude: next_val()?.parse()?,
            zoom: next_val()?.parse()?,
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
