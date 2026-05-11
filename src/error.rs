use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum WeatherError {
    #[error("Network error: {message}")]
    Network { message: String },

    #[error("API error (HTTP {status_code}): {body}")]
    Api { status_code: u16, body: String },

    #[error("Parse error for field '{field}': {message}")]
    Parse { message: String, field: String },

    #[error("Validation error for '{field}': {constraint}")]
    Validation { field: String, constraint: String },

    #[error("Storage error at {path}: {message}")]
    Storage { path: PathBuf, message: String },

    #[error("Configuration error: {detail}")]
    Config { detail: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_error_display() {
        let err = WeatherError::Network {
            message: "Connection refused".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Network error"));
        assert!(msg.contains("Connection refused"));
    }

    #[test]
    fn test_api_error_display() {
        let err = WeatherError::Api {
            status_code: 429,
            body: "Rate limit exceeded".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("API error"));
        assert!(msg.contains("429"));
        assert!(msg.contains("Rate limit exceeded"));
    }

    #[test]
    fn test_parse_error_display() {
        let err = WeatherError::Parse {
            message: "invalid type".to_string(),
            field: "temperature".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Parse error"));
        assert!(msg.contains("temperature"));
        assert!(msg.contains("invalid type"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = WeatherError::Validation {
            field: "latitude".to_string(),
            constraint: "Must be between -90 and 90".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Validation error"));
        assert!(msg.contains("latitude"));
        assert!(msg.contains("Must be between -90 and 90"));
    }

    #[test]
    fn test_storage_error_display() {
        let err = WeatherError::Storage {
            path: PathBuf::from("/tmp/test.json"),
            message: "Permission denied".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Storage error"));
        assert!(msg.contains("/tmp/test.json"));
        assert!(msg.contains("Permission denied"));
    }

    #[test]
    fn test_config_error_display() {
        let err = WeatherError::Config {
            detail: "Missing API key".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("Missing API key"));
    }
}
