use crate::ParseBoundsError::{BadLen, ParseCoordError};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Debug, Copy, Clone)]
pub struct Bounds {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Bounds {
    /// Create a new Bounds object.
    pub fn new(left: f64, bottom: f64, right: f64, top: f64) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }

    /// Create maximum bounds object in WGS84 space.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::MAX,
    ///     Bounds::new(-180.0, -90.0, 180.0, 90.0)
    /// );
    /// ```
    pub const MAX: Self = {
        Self {
            left: -180.0,
            bottom: -90.0,
            right: 180.0,
            top: 90.0,
        }
    };

    /// Create maximum bounds object usable with vector tiles.
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds>
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::MAX_TILED,
    ///     Bounds::new(-180.0, -85.05112877980659, 180.0, 85.0511287798066)
    /// );
    /// ```
    pub const MAX_TILED: Self = {
        Self {
            left: -180.0,
            bottom: -85.05112877980659,
            right: 180.0,
            top: 85.0511287798066,
        }
    };
}

impl Default for Bounds {
    /// Default bounds are set to `[-180, -85.05112877980659, 180, 85.0511287798066]`
    /// See <https://github.com/mapbox/tilejson-spec/tree/master/3.0.0#35-bounds>
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(Bounds::MAX_TILED, Bounds::default());
    /// ```
    fn default() -> Self {
        Self::MAX_TILED
    }
}

impl Add for Bounds {
    type Output = Bounds;

    /// Combine two bounds, resulting in an bounding box that encloses both.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 3., 7., 9.) + Bounds::new(2., 2., 8., 8.),
    ///     Bounds::new(1., 2., 8., 9.)
    /// )
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            if self.left < rhs.left {
                self.left
            } else {
                rhs.left
            },
            if self.bottom < rhs.bottom {
                self.bottom
            } else {
                rhs.bottom
            },
            if self.right > rhs.right {
                self.right
            } else {
                rhs.right
            },
            if self.top > rhs.top {
                self.top
            } else {
                rhs.top
            },
        )
    }
}

impl AddAssign for Bounds {
    /// Combine another bounds into this one, resulting in an bounding box that encloses both.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// let mut value = Bounds::new(1., 3., 7., 9.);
    /// value += Bounds::new(2., 2., 8., 8.);
    /// assert_eq!(value, Bounds::new(1., 2., 8., 9.))
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        if self.left > rhs.left {
            self.left = rhs.left
        }
        if self.bottom > rhs.bottom {
            self.bottom = rhs.bottom
        }
        if self.right < rhs.right {
            self.right = rhs.right
        }
        if self.top < rhs.top {
            self.top = rhs.top
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseBoundsError {
    /// Incorrect number of values
    BadLen,
    /// Wrapped error from the `parse::<f64>()`
    ParseCoordError(ParseFloatError),
}

impl Display for ParseBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BadLen => f.write_str("Incorrect number of values. Bounds expects four f64 values."),
            ParseCoordError(e) => e.fmt(f),
        }
    }
}

impl From<[f64; 4]> for Bounds {
    /// Parse four f64 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::from([1., 2., 3., 4.])
    /// );
    /// ```
    fn from(value: [f64; 4]) -> Self {
        Self {
            left: value[0],
            bottom: value[1],
            right: value[2],
            top: value[3],
        }
    }
}

impl From<[f32; 4]> for Bounds {
    /// Parse four f32 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::from([1.0f32, 2.0f32, 3.0f32, 4.0f32])
    /// );
    /// ```
    fn from(value: [f32; 4]) -> Self {
        Self {
            left: value[0] as f64,
            bottom: value[1] as f64,
            right: value[2] as f64,
            top: value[3] as f64,
        }
    }
}

impl From<[i32; 4]> for Bounds {
    /// Parse four i32 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::from([1, 2, 3, 4])
    /// );
    /// ```
    fn from(value: [i32; 4]) -> Self {
        Self {
            left: value[0] as f64,
            bottom: value[1] as f64,
            right: value[2] as f64,
            top: value[3] as f64,
        }
    }
}

impl TryFrom<Vec<f64>> for Bounds {
    type Error = ParseBoundsError;

    /// Parse four f64 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::try_from(vec![1., 2., 3., 4.]).unwrap()
    /// );
    /// ```
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        let arr: [f64; 4] = value.try_into().map_err(|_| BadLen)?;
        Ok(arr.into())
    }
}

impl TryFrom<&[f64]> for Bounds {
    type Error = ParseBoundsError;

    /// Parse four f64 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::try_from(vec![1., 2., 3., 4.].as_slice()).unwrap()
    /// );
    /// ```
    fn try_from(value: &[f64]) -> Result<Self, Self::Error> {
        let arr: [f64; 4] = value.try_into().map_err(|_| BadLen)?;
        Ok(arr.into())
    }
}

impl TryFrom<&[f32]> for Bounds {
    type Error = ParseBoundsError;

    /// Parse four f32 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::try_from(vec![1.0f32, 2.0f32, 3.0f32, 4.0f32].as_slice()).unwrap()
    /// );
    /// ```
    fn try_from(value: &[f32]) -> Result<Self, Self::Error> {
        let arr: [f32; 4] = value.try_into().map_err(|_| BadLen)?;
        Ok(arr.into())
    }
}

impl TryFrom<&[i32]> for Bounds {
    type Error = ParseBoundsError;

    /// Parse four i32 values as a Bounds value, same order as the [Bounds::new] constructor.
    ///
    /// ```
    /// # use tilejson::Bounds;
    /// assert_eq!(
    ///     Bounds::new(1., 2., 3., 4.),
    ///     Bounds::try_from(vec![1, 2, 3, 4].as_slice()).unwrap()
    /// );
    /// ```
    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        let arr: [i32; 4] = value.try_into().map_err(|_| BadLen)?;
        Ok(arr.into())
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
        let mut values = s.split(',');
        let mut result = [0.; 4];
        for val in &mut result {
            *val = values
                .next()
                .ok_or(ParseBoundsError::BadLen)?
                .trim()
                .parse()
                .map_err(ParseBoundsError::ParseCoordError)?;
        }
        values
            .next()
            .map_or(Ok(result.into()), |_| Err(ParseBoundsError::BadLen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseBoundsError::BadLen;

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

    #[test]
    fn test_parse_errors() {
        let err = |s| Bounds::from_str(s).unwrap_err();
        assert_eq!(err("0,0,0"), BadLen);
        assert_eq!(err("0,0,0,0,0"), BadLen);
        assert!(matches!(err(""), ParseCoordError(_)));
        assert!(matches!(err("a"), ParseCoordError(_)));
        assert!(matches!(err("0,0,0,1a"), ParseCoordError(_)));
    }

    #[test]
    fn test_from() -> Result<(), ParseBoundsError> {
        let exp = Bounds::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(exp, Bounds::from([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(exp, Bounds::try_from([1.0, 2.0, 3.0, 4.0].as_slice())?);
        assert_eq!(exp, Bounds::try_from(vec![1.0, 2.0, 3.0, 4.0])?);
        let val = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(exp, Bounds::try_from(val.as_slice())?);
        assert_eq!(exp, Bounds::try_from(val.as_slice())?);

        // f32
        assert_eq!(exp, Bounds::from([1.0f32, 2.0f32, 3.0f32, 4.0f32]));
        let val_array = [1.0f32, 2.0f32, 3.0f32, 4.0f32];
        assert_eq!(exp, Bounds::try_from(val_array.as_slice())?);
        let val = vec![1.0f32, 2.0f32, 3.0f32, 4.0f32];
        let borrowed = &val;
        assert_eq!(exp, Bounds::try_from(borrowed.as_slice())?);
        assert_eq!(exp, Bounds::try_from(val.as_slice())?);

        // i32
        assert_eq!(exp, Bounds::from([1, 2, 3, 4]));
        assert_eq!(exp, Bounds::try_from([1, 2, 3, 4].as_slice())?);
        let val = vec![1, 2, 3, 4];
        let borrowed = &val;
        assert_eq!(exp, Bounds::try_from(borrowed.as_slice())?);
        assert_eq!(exp, Bounds::try_from(val.as_slice())?);
        Ok(())
    }
}
