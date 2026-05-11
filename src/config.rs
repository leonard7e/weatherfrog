pub const API_BASE_URL: &str = "https://api.open-meteo.com/v1";
pub const GEOCODING_BASE_URL: &str = "https://geocoding-api.open-meteo.com/v1";

#[allow(dead_code)]
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERAL_ERROR: i32 = 1;
pub const EXIT_NETWORK_ERROR: i32 = 2;
pub const EXIT_API_ERROR: i32 = 3;
pub const EXIT_LOCATION_NOT_FOUND: i32 = 4;
pub const EXIT_INVALID_PARAMETER: i32 = 5;
#[allow(dead_code)]
pub const EXIT_DUPLICATE_FAVORITE: i32 = 6;
#[allow(dead_code)]
pub const EXIT_NO_FAVORITES: i32 = 7;
#[allow(dead_code)]
pub const EXIT_FAVORITE_NOT_FOUND: i32 = 8;
#[allow(dead_code)]
pub const DEFAULT_FORECAST_DAYS: u8 = 3;
pub const MAX_FORECAST_DAYS: u8 = 16;
pub const MIN_FORECAST_DAYS: u8 = 1;
#[allow(dead_code)]
pub const RETRY_MAX_ATTEMPTS: u8 = 3;
#[allow(dead_code)]
pub const RETRY_INITIAL_DELAY_SECS: u64 = 1;
