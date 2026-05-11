use crate::models::{ForecastData, WeatherData};

pub fn format_weather_json(data: &WeatherData) -> Result<String, crate::error::WeatherError> {
    serde_json::to_string_pretty(data).map_err(|e| crate::error::WeatherError::Parse {
        message: format!("Failed to serialize weather data: {}", e),
        field: "json_output".to_string(),
    })
}

pub fn format_forecast_json(data: &ForecastData) -> Result<String, crate::error::WeatherError> {
    serde_json::to_string_pretty(data).map_err(|e| crate::error::WeatherError::Parse {
        message: format!("Failed to serialize forecast data: {}", e),
        field: "json_output".to_string(),
    })
}
