use crate::error::WeatherError;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl Location {
    pub fn new(
        name: String,
        latitude: f64,
        longitude: f64,
        country: Option<String>,
    ) -> Result<Self, WeatherError> {
        let trimmed = name.trim().to_string();
        if trimmed.is_empty() {
            return Err(WeatherError::Validation {
                field: "name".to_string(),
                constraint: "Location name must not be empty".to_string(),
            });
        }
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(WeatherError::Validation {
                field: "latitude".to_string(),
                constraint: "Latitude must be between -90 and 90".to_string(),
            });
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(WeatherError::Validation {
                field: "longitude".to_string(),
                constraint: "Longitude must be between -180 and 180".to_string(),
            });
        }
        Ok(Self {
            name: trimmed,
            latitude,
            longitude,
            country,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_location() {
        let loc = Location::new("London".into(), 51.5, -0.13, None).unwrap();
        assert_eq!(loc.name, "London");
        assert_eq!(loc.latitude, 51.5);
        assert_eq!(loc.longitude, -0.13);
    }

    #[test]
    fn test_empty_name_rejected() {
        let err = Location::new("".into(), 0.0, 0.0, None).unwrap_err();
        assert!(matches!(err, WeatherError::Validation { field, .. } if field == "name"));
    }

    #[test]
    fn test_whitespace_name_trimmed() {
        let loc = Location::new("  Paris  ".into(), 48.8, 2.3, None).unwrap();
        assert_eq!(loc.name, "Paris");
    }

    #[test]
    fn test_invalid_latitude_too_low() {
        let err = Location::new("Test".into(), -100.0, 0.0, None).unwrap_err();
        assert!(matches!(err, WeatherError::Validation { field, .. } if field == "latitude"));
    }

    #[test]
    fn test_invalid_latitude_too_high() {
        let err = Location::new("Test".into(), 100.0, 0.0, None).unwrap_err();
        assert!(matches!(err, WeatherError::Validation { field, .. } if field == "latitude"));
    }

    #[test]
    fn test_invalid_longitude_too_low() {
        let err = Location::new("Test".into(), 0.0, -200.0, None).unwrap_err();
        assert!(matches!(err, WeatherError::Validation { field, .. } if field == "longitude"));
    }

    #[test]
    fn test_invalid_longitude_too_high() {
        let err = Location::new("Test".into(), 0.0, 200.0, None).unwrap_err();
        assert!(matches!(err, WeatherError::Validation { field, .. } if field == "longitude"));
    }

    #[test]
    fn test_temperature_unit_display() {
        assert_eq!(TemperatureUnit::Celsius.as_str(), "°C");
        assert_eq!(TemperatureUnit::Fahrenheit.as_str(), "°F");
    }
}
