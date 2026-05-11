use crate::error::WeatherError;
use chrono::{DateTime, FixedOffset, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct GeocodingResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct WeatherApiResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub current: Option<CurrentWeather>,
    pub current_units: Option<CurrentUnits>,
    pub daily: Option<DailyForecast>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentUnits {
    #[serde(rename = "temperature_2m")]
    pub temperature: String,
    #[serde(rename = "relative_humidity_2m")]
    pub humidity: String,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentWeather {
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
    #[serde(rename = "relative_humidity_2m")]
    pub humidity: f64,
    #[serde(rename = "apparent_temperature")]
    pub apparent_temperature: f64,
    #[serde(rename = "weather_code")]
    pub weather_code: u16,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed: f64,
    #[serde(rename = "wind_direction_10m")]
    pub wind_direction: Option<f64>,
    #[serde(rename = "pressure_msl")]
    pub pressure: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    #[serde(rename = "temperature_2m_max")]
    pub temperature_max: Vec<f64>,
    #[serde(rename = "temperature_2m_min")]
    pub temperature_min: Vec<f64>,
    #[serde(rename = "weather_code")]
    pub weather_code: Vec<u16>,
    #[serde(rename = "relative_humidity_2m_mean")]
    pub humidity_mean: Option<Vec<f64>>,
    #[serde(rename = "wind_speed_10m_max")]
    pub wind_speed_max: Option<Vec<f64>>,
    #[serde(rename = "precipitation_probability_max")]
    pub precipitation_probability: Option<Vec<f64>>,
}

pub fn weather_code_to_string(code: u16) -> &'static str {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 | 48 => "Foggy",
        51 => "Light drizzle",
        53 => "Moderate drizzle",
        55 => "Dense drizzle",
        56 | 57 => "Freezing drizzle",
        61 => "Slight rain",
        63 => "Moderate rain",
        65 => "Heavy rain",
        66 | 67 => "Freezing rain",
        71 => "Slight snow",
        73 => "Moderate snow",
        75 => "Heavy snow",
        77 => "Snow grains",
        80 => "Slight rain showers",
        81 => "Moderate rain showers",
        82 => "Violent rain showers",
        85 => "Slight snow showers",
        86 => "Heavy snow showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => "Unknown",
    }
}

pub fn wind_direction_to_string(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    directions[index].to_string()
}

pub fn parse_api_timestamp(time_str: &str) -> Result<DateTime<Utc>, WeatherError> {
    let full = format!("{}:00Z", time_str);
    if let Ok(dt) = DateTime::parse_from_str(&full, "%Y-%m-%dT%H:%M:%S%#z") {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = time_str.parse::<DateTime<FixedOffset>>() {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = time_str.parse::<DateTime<Utc>>() {
        return Ok(dt);
    }
    Err(WeatherError::Parse {
        message: format!("Cannot parse timestamp '{}'", time_str),
        field: "timestamp".to_string(),
    })
}
