use crate::error::WeatherError;
use crate::models::Location;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(default)]
    pub default_unit: String,
    #[serde(default = "default_days")]
    pub default_days: usize,
    #[serde(default)]
    pub favorites: Vec<Location>,
}

fn default_days() -> usize {
    3
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_unit: "celsius".to_string(),
            default_days: 3,
            favorites: Vec::new(),
        }
    }
}

fn prefs_path() -> Result<PathBuf, WeatherError> {
    let base = dirs::home_dir().ok_or_else(|| WeatherError::Config {
        detail: "Cannot determine home directory".to_string(),
    })?;
    Ok(base.join(".weatherfrog").join("favorites.json"))
}

pub fn load_preferences() -> Result<UserPreferences, WeatherError> {
    let path = prefs_path()?;
    if !path.exists() {
        return Ok(UserPreferences::default());
    }
    let content = std::fs::read_to_string(&path).map_err(|e| WeatherError::Storage {
        path: path.clone(),
        message: format!("Failed to read favorites: {}", e),
    })?;
    serde_json::from_str(&content).map_err(|e| WeatherError::Parse {
        message: format!("Failed to parse favorites: {}", e),
        field: "favorites.json".to_string(),
    })
}

pub fn save_preferences(prefs: &UserPreferences) -> Result<(), WeatherError> {
    let path = prefs_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| WeatherError::Storage {
            path: parent.to_path_buf(),
            message: format!("Failed to create favorites directory: {}", e),
        })?;
    }
    let content = serde_json::to_string_pretty(prefs).map_err(|e| WeatherError::Parse {
        message: format!("Failed to serialize favorites: {}", e),
        field: "favorites.json".to_string(),
    })?;
    std::fs::write(&path, content).map_err(|e| WeatherError::Storage {
        path: path.clone(),
        message: format!("Failed to write favorites: {}", e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_preferences_default() {
        let prefs = UserPreferences::default();
        assert_eq!(prefs.default_unit, "celsius");
        assert_eq!(prefs.default_days, 3);
        assert!(prefs.favorites.is_empty());
    }

    #[test]
    fn test_user_preferences_serde_roundtrip() {
        let fav = Location::new("Paris".into(), 48.8, 2.3, Some("France".into())).unwrap();
        let prefs = UserPreferences {
            default_unit: "fahrenheit".to_string(),
            default_days: 5,
            favorites: vec![fav],
        };
        let json = serde_json::to_string(&prefs).unwrap();
        let deserialized: UserPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(prefs.default_unit, deserialized.default_unit);
        assert_eq!(prefs.default_days, deserialized.default_days);
        assert_eq!(prefs.favorites.len(), deserialized.favorites.len());
        assert_eq!(prefs.favorites[0].name, deserialized.favorites[0].name);
    }

    #[test]
    fn test_user_preferences_empty_favorites() {
        let json = r#"{"default_unit":"celsius","default_days":3,"favorites":[]}"#;
        let prefs: UserPreferences = serde_json::from_str(json).unwrap();
        assert!(prefs.favorites.is_empty());
    }

    #[test]
    fn test_user_preferences_duplicate_detection() {
        let paris = Location::new("Paris".into(), 48.8, 2.3, None).unwrap();
        let paris_dup = Location::new("Paris".into(), 48.8, 2.3, None).unwrap();
        let mut prefs = UserPreferences::default();
        prefs.favorites.push(paris);
        let is_duplicate = prefs
            .favorites
            .iter()
            .any(|f| f.name.to_lowercase() == paris_dup.name.to_lowercase());
        assert!(is_duplicate);
    }
}
