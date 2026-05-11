pub mod json;
mod text;

pub use text::{format_forecast_text, format_weather_text};

pub use json::{format_forecast_json, format_weather_json};
