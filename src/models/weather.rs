use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeatherData {
    pub location: crate::models::Location,
    pub temperature: f64,
    pub temperature_unit: crate::models::TemperatureUnit,
    pub conditions: String,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_direction: Option<String>,
    pub pressure: f64,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Location, TemperatureUnit};

    fn sample_weather() -> WeatherData {
        WeatherData {
            location: Location::new("London".into(), 51.5, -0.13, Some("UK".into())).unwrap(),
            temperature: 15.3,
            temperature_unit: TemperatureUnit::Celsius,
            conditions: "Partly cloudy".into(),
            humidity: 72.0,
            wind_speed: 12.5,
            wind_direction: Some("NW".into()),
            pressure: 1013.0,
            timestamp: DateTime::parse_from_rfc3339("2026-05-11T14:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
        }
    }

    #[test]
    fn test_weather_serde_roundtrip() {
        let w = sample_weather();
        let json = serde_json::to_string(&w).unwrap();
        let deserialized: WeatherData = serde_json::from_str(&json).unwrap();
        assert_eq!(w, deserialized);
    }

    #[test]
    fn test_weather_json_contains_fields() {
        let w = sample_weather();
        let json = serde_json::to_string(&w).unwrap();
        assert!(json.contains("temperature"));
        assert!(json.contains("humidity"));
        assert!(json.contains("wind_speed"));
        assert!(json.contains("pressure"));
        assert!(json.contains("conditions"));
        assert!(json.contains("temperature_unit"));
        assert!(json.contains("timestamp"));
    }

    #[test]
    fn test_weather_json_valid() {
        let w = sample_weather();
        let json = serde_json::to_string(&w).unwrap();
        let result: Result<serde_json::Value, _> = serde_json::from_str(&json);
        assert!(result.is_ok());
    }
}
