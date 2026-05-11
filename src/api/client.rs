use crate::api::types::{weather_code_to_string, wind_direction_to_string};
use crate::config;
use crate::error::WeatherError;
use crate::models::{ForecastData, ForecastDay, Location, TemperatureUnit, WeatherData};
use chrono::NaiveDate;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    async fn get_with_retry(&self, url: &str) -> Result<reqwest::Response, WeatherError> {
        let mut last_error = None;
        let mut delay = config::RETRY_INITIAL_DELAY_SECS;

        for attempt in 0..config::RETRY_MAX_ATTEMPTS {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_server_error()
                        && attempt + 1 < config::RETRY_MAX_ATTEMPTS
                    {
                        let status = response.status().as_u16();
                        let body = response.text().await.unwrap_or_default();
                        last_error = Some(WeatherError::Api {
                            status_code: status,
                            body,
                        });
                        tokio::time::sleep(Duration::from_secs(delay)).await;
                        delay *= 2;
                        continue;
                    }
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(WeatherError::Network {
                        message: format!(
                            "Request failed (attempt {}/{}): {}",
                            attempt + 1,
                            config::RETRY_MAX_ATTEMPTS,
                            e
                        ),
                    });
                    if attempt + 1 < config::RETRY_MAX_ATTEMPTS {
                        tokio::time::sleep(Duration::from_secs(delay)).await;
                        delay *= 2;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| WeatherError::Network {
            message: "Request failed after all retry attempts".to_string(),
        }))
    }

    pub async fn geocode(
        &self,
        city: &str,
    ) -> Result<Vec<crate::api::GeocodingResult>, WeatherError> {
        let url = format!(
            "{}/search?name={}&count=10&language=en&format=json",
            config::GEOCODING_BASE_URL,
            urlencoding(city)
        );

        let response = self.get_with_retry(&url).await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(WeatherError::Api {
                status_code: status.as_u16(),
                body,
            });
        }

        let text = response.text().await.map_err(|e| WeatherError::Network {
            message: format!("Failed to read geocoding response: {}", e),
        })?;

        let geo: crate::api::GeocodingResponse =
            serde_json::from_str(&text).map_err(|e| WeatherError::Parse {
                message: format!("Failed to parse geocoding response: {}", e),
                field: "geocoding_response".to_string(),
            })?;

        Ok(geo.results.unwrap_or_default())
    }

    pub async fn fetch_current(&self, location: &Location) -> Result<WeatherData, WeatherError> {
        let url = format!(
            "{}/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,wind_speed_10m,wind_direction_10m,pressure_msl",
            config::API_BASE_URL,
            location.latitude,
            location.longitude
        );

        let response = self.get_with_retry(&url).await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(WeatherError::Api {
                status_code: status.as_u16(),
                body,
            });
        }

        let text = response.text().await.map_err(|e| WeatherError::Network {
            message: format!("Failed to read weather response: {}", e),
        })?;

        let api_response: crate::api::WeatherApiResponse =
            serde_json::from_str(&text).map_err(|e| WeatherError::Parse {
                message: format!("Failed to parse weather data: {}", e),
                field: "weather_api_response".to_string(),
            })?;

        let current = api_response.current.ok_or_else(|| WeatherError::Parse {
            message: "Missing current weather data in API response".to_string(),
            field: "current".to_string(),
        })?;

        let wind_dir = current.wind_direction.map(wind_direction_to_string);

        let timestamp = crate::api::types::parse_api_timestamp(&current.time)?;

        Ok(WeatherData {
            location: location.clone(),
            temperature: current.temperature,
            temperature_unit: TemperatureUnit::Celsius,
            conditions: weather_code_to_string(current.weather_code).to_string(),
            humidity: current.humidity,
            wind_speed: current.wind_speed,
            wind_direction: wind_dir,
            pressure: current.pressure.unwrap_or(1013.0),
            timestamp,
        })
    }

    pub async fn fetch_forecast(
        &self,
        location: &Location,
        days: u8,
    ) -> Result<ForecastData, WeatherError> {
        let days = days.clamp(config::MIN_FORECAST_DAYS, config::MAX_FORECAST_DAYS);
        let url = format!(
            "{}/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,weather_code,relative_humidity_2m_mean,wind_speed_10m_max,precipitation_probability_max&forecast_days={}",
            config::API_BASE_URL,
            location.latitude,
            location.longitude,
            days
        );

        let response = self.get_with_retry(&url).await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(WeatherError::Api {
                status_code: status.as_u16(),
                body,
            });
        }

        let text = response.text().await.map_err(|e| WeatherError::Network {
            message: format!("Failed to read forecast response: {}", e),
        })?;

        let api_response: crate::api::WeatherApiResponse =
            serde_json::from_str(&text).map_err(|e| WeatherError::Parse {
                message: format!("Failed to parse forecast data: {}", e),
                field: "forecast_api_response".to_string(),
            })?;

        let daily = api_response.daily.ok_or_else(|| WeatherError::Parse {
            message: "Missing daily forecast data in API response".to_string(),
            field: "daily".to_string(),
        })?;

        let mut forecast_days = Vec::with_capacity(daily.time.len());
        for i in 0..daily.time.len() {
            let date = NaiveDate::parse_from_str(&daily.time[i], "%Y-%m-%d").map_err(|e| {
                WeatherError::Parse {
                    message: format!("Failed to parse date '{}': {}", daily.time[i], e),
                    field: format!("daily.time[{}]", i),
                }
            })?;

            let humidity = daily.humidity_mean.as_ref().and_then(|h| h.get(i).copied());
            let wind_speed = daily
                .wind_speed_max
                .as_ref()
                .and_then(|w| w.get(i).copied());
            let precip = daily
                .precipitation_probability
                .as_ref()
                .and_then(|p| p.get(i).copied());

            forecast_days.push(ForecastDay {
                date,
                temperature_max: daily.temperature_max[i],
                temperature_min: daily.temperature_min[i],
                conditions: weather_code_to_string(daily.weather_code[i]).to_string(),
                humidity,
                wind_speed,
                precipitation_probability: precip,
            });
        }

        Ok(ForecastData {
            location: location.clone(),
            days: forecast_days,
            temperature_unit: TemperatureUnit::Celsius,
        })
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            other => format!("%{:02X}", other as u8),
        })
        .collect()
}
