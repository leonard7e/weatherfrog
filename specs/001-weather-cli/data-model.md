# Data Model: WeatherFrog CLI

**Date**: 2026-05-11  
**Feature**: 001-weather-cli

## Domain Entities

### WeatherData

Represents current weather conditions for a specific location.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `location` | `Location` | Geographic location | Required |
| `temperature` | `f64` | Current temperature in configured unit | Must be finite |
| `temperature_unit` | `TemperatureUnit` | Celsius or Fahrenheit | Required |
| `conditions` | `String` | Weather condition description (e.g., "Partly cloudy") | Non-empty |
| `humidity` | `f64` | Relative humidity percentage | 0.0 – 100.0 |
| `wind_speed` | `f64` | Wind speed in km/h | Non-negative |
| `pressure` | `f64` | Atmospheric pressure in hPa | Positive |
| `timestamp` | `DateTime<Utc>` | Observation time | Required |

**Validation rules**:
- Temperature must be a finite number (not NaN or Infinity)
- Humidity must be between 0 and 100 inclusive
- Wind speed must be non-negative
- Pressure must be positive

### ForecastData

Represents multi-day weather forecast for a specific location.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `location` | `Location` | Geographic location | Required |
| `days` | `Vec<ForecastDay>` | Daily forecast entries | Non-empty |
| `temperature_unit` | `TemperatureUnit` | Celsius or Fahrenheit | Required |

### ForecastDay

Represents a single day's forecast data.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `date` | `NaiveDate` | Forecast date | Required |
| `temperature_max` | `f64` | Daily high temperature | Must be finite |
| `temperature_min` | `f64` | Daily low temperature | Must be finite |
| `conditions` | `String` | Weather condition description | Non-empty |
| `humidity` | `f64` | Daily average humidity | 0.0 – 100.0 |
| `wind_speed` | `f64` | Daily average wind speed | Non-negative |
| `precipitation_probability` | `f64` | Chance of precipitation | 0.0 – 100.0 |

**Validation rules**:
- `temperature_max` must be >= `temperature_min`
- All numeric fields must be finite
- Date must be today or in the future

### Location

Represents a geographic location by name or coordinates.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `name` | `String` | City or location name | Non-empty |
| `latitude` | `f64` | Geographic latitude | -90.0 – 90.0 |
| `longitude` | `f64` | Geographic longitude | -180.0 – 180.0 |

**Validation rules**:
- Name must be non-empty and trimmed
- Latitude must be within valid range
- Longitude must be within valid range

### TemperatureUnit

Enumeration for temperature display units.

| Variant | Description |
|---------|-------------|
| `Celsius` | Temperature in degrees Celsius |
| `Fahrenheit` | Temperature in degrees Fahrenheit |

### WeatherError

Custom error type for all failure modes in the application.

| Variant | Description | Contains |
|---------|-------------|----------|
| `Network` | HTTP request or connection failure | Source error, optional user message |
| `Api` | API returned non-success status | Status code, response body |
| `Parse` | Failed to parse API response | Source error, field name |
| `Validation` | Input data failed validation | Field name, constraint description |
| `Storage` | File I/O operation failed | File path, source error |
| `Config` | Application configuration error | Missing key or invalid value |

**Error handling rules**:
- All variants implement `std::error::Error`, `std::fmt::Display`, and `std::fmt::Debug`
- Each variant provides a user-friendly error message
- No variant should cause a panic; all errors are recoverable

### UserPreferences

Stores user-specific settings and saved favorites.

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `default_unit` | `TemperatureUnit` | Preferred temperature unit | `Celsius` |
| `default_days` | `usize` | Default forecast days | `3` |
| `favorites` | `Vec<Location>` | Saved favorite locations | Empty |

**Storage**: Serialized to JSON at `~/.weatherfrog/favorites.json`

**Validation rules**:
- `default_days` must be between 1 and 16 inclusive
- `favorites` list must not contain duplicate location names (case-insensitive)

## State Transitions

### Location Resolution Flow

```
User Input → Parse → Validate → Geocode (if needed) → API Request → Response
```

1. User provides city name or coordinates
2. Input is parsed into a `Location` candidate
3. Location is validated (name non-empty, coordinates in range)
4. If city name: geocoding API resolves to coordinates
5. Weather API request is made with coordinates
6. Response is parsed into `WeatherData` or `ForecastData`

### Error Propagation Flow

```
Operation → Result<T, WeatherError> → ? operator → Caller handles or propagates
```

All fallible operations return `Result<T, WeatherError>`. Errors propagate via the `?` operator until reaching the top-level command handler, which converts the error to a user-friendly message and appropriate exit code.

## Relationships

- `WeatherData` **has-a** `Location`
- `ForecastData` **has-a** `Location` and **has-many** `ForecastDay`
- `UserPreferences` **has-many** `Location` (as favorites)
- All entities are independent and can be created/tested in isolation
