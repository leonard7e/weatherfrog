use crate::models::{ForecastData, WeatherData};

pub fn format_weather_text(data: &WeatherData) -> String {
    let country = data.location.country.as_deref().unwrap_or("");
    let location_display = if country.is_empty() {
        data.location.name.clone()
    } else {
        format!("{}, {}", data.location.name, country)
    };

    let wind_dir = data.wind_direction.as_deref().unwrap_or("");
    let wind_display = if wind_dir.is_empty() {
        format!("{:.1} km/h", data.wind_speed)
    } else {
        format!("{:.1} km/h {}", data.wind_speed, wind_dir)
    };

    format!(
        "Weather for {}
──────────────────────────────────
Temperature:    {:.1} {}
Conditions:     {}
Humidity:       {:.0}%
Wind:           {}
Pressure:       {:.0} hPa
Observed:       {}",
        location_display,
        data.temperature,
        data.temperature_unit.as_str(),
        data.conditions,
        data.humidity,
        wind_display,
        data.pressure,
        data.timestamp.format("%Y-%m-%d %H:%M UTC"),
    )
}

pub fn format_forecast_text(data: &ForecastData) -> String {
    let country = data.location.country.as_deref().unwrap_or("");
    let location_display = if country.is_empty() {
        data.location.name.clone()
    } else {
        format!("{}, {}", data.location.name, country)
    };

    let unit = data.temperature_unit.as_str();
    let mut output = format!(
        "Forecast for {} ({} days)
─────────────────────────────────────────────────────────────
Date         High    Low     Conditions        Humidity  Wind
─────────────────────────────────────────────────────────────",
        location_display,
        data.days.len()
    );

    for day in &data.days {
        let hum = day
            .humidity
            .map(|h| format!("{:.0}%", h))
            .unwrap_or_else(|| "N/A".to_string());
        let wind = day
            .wind_speed
            .map(|w| format!("{:.0} km/h", w))
            .unwrap_or_else(|| "N/A".to_string());
        output.push_str(&format!(
            "\n{}   {:.0} {} {:.0} {}   {:<18} {:<9} {}",
            day.date,
            day.temperature_max,
            unit,
            day.temperature_min,
            unit,
            day.conditions,
            hum,
            wind,
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ForecastDay, Location, TemperatureUnit, WeatherData};
    use chrono::{DateTime, Utc};

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

    fn sample_forecast() -> ForecastData {
        ForecastData {
            location: Location::new("Berlin".into(), 52.5, 13.4, Some("Germany".into())).unwrap(),
            days: vec![ForecastDay {
                date: chrono::NaiveDate::from_ymd_opt(2026, 5, 11).unwrap(),
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
    fn test_weather_text_contains_city() {
        let text = format_weather_text(&sample_weather());
        assert!(text.contains("London"));
        assert!(text.contains("UK"));
    }

    #[test]
    fn test_weather_text_contains_fields() {
        let text = format_weather_text(&sample_weather());
        assert!(text.contains("Temperature"));
        assert!(text.contains("Conditions"));
        assert!(text.contains("Humidity"));
        assert!(text.contains("Wind"));
        assert!(text.contains("Pressure"));
        assert!(text.contains("Observed"));
    }

    #[test]
    fn test_weather_text_contains_unit() {
        let text = format_weather_text(&sample_weather());
        assert!(text.contains("°C"));
    }

    #[test]
    fn test_weather_text_contains_values() {
        let text = format_weather_text(&sample_weather());
        assert!(text.contains("15.3"));
        assert!(text.contains("Partly cloudy"));
        assert!(text.contains("72%"));
    }

    #[test]
    fn test_forecast_text_contains_city() {
        let text = format_forecast_text(&sample_forecast());
        assert!(text.contains("Berlin"));
    }

    #[test]
    fn test_forecast_text_contains_headers() {
        let text = format_forecast_text(&sample_forecast());
        assert!(text.contains("Date"));
        assert!(text.contains("High"));
        assert!(text.contains("Low"));
    }
}
