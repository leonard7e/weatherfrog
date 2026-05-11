use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ForecastData {
    pub location: crate::models::Location,
    pub days: Vec<ForecastDay>,
    pub temperature_unit: crate::models::TemperatureUnit,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ForecastDay {
    pub date: NaiveDate,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub conditions: String,
    pub humidity: Option<f64>,
    pub wind_speed: Option<f64>,
    pub precipitation_probability: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Location, TemperatureUnit};
    use chrono::NaiveDate;

    fn sample_forecast() -> ForecastData {
        ForecastData {
            location: Location::new("Berlin".into(), 52.5, 13.4, Some("Germany".into())).unwrap(),
            days: vec![ForecastDay {
                date: NaiveDate::from_ymd_opt(2026, 5, 11).unwrap(),
                temperature_max: 18.0,
                temperature_min: 9.0,
                conditions: "Sunny".into(),
                humidity: Some(45.0),
                wind_speed: Some(8.0),
                precipitation_probability: Some(10.0),
            }],
            temperature_unit: TemperatureUnit::Celsius,
        }
    }

    #[test]
    fn test_forecast_max_gte_min() {
        let f = sample_forecast();
        for day in &f.days {
            assert!(day.temperature_max >= day.temperature_min);
        }
    }

    #[test]
    fn test_forecast_serde_roundtrip() {
        let f = sample_forecast();
        let json = serde_json::to_string(&f).unwrap();
        let deserialized: ForecastData = serde_json::from_str(&json).unwrap();
        assert_eq!(f.location, deserialized.location);
        assert_eq!(f.days.len(), deserialized.days.len());
        assert_eq!(f.days[0].date, deserialized.days[0].date);
    }

    #[test]
    fn test_forecast_json_valid() {
        let f = sample_forecast();
        let json = serde_json::to_string(&f).unwrap();
        let result: Result<serde_json::Value, _> = serde_json::from_str(&json);
        assert!(result.is_ok());
    }
}
