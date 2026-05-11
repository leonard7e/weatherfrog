# CLI Command Contracts

**Date**: 2026-05-11  
**Feature**: 001-weather-cli

## Overview

WeatherFrog uses a subcommand-based CLI structure. Each subcommand has defined arguments, flags, output formats, and exit codes.

---

## Command: `weatherfrog fetch`

**Purpose**: Fetch current weather for a location.

### Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--city` | `-c` | Mutually exclusive with `--favorite` | City name to fetch weather for |
| `--latitude` | `--lat` | Mutually exclusive with `--city` and `--favorite` | Geographic latitude |
| `--longitude` | `--lon` | Required with `--latitude` | Geographic longitude |
| `--favorite` | `-f` | Mutually exclusive with `--city` and `--latitude` | Name of a saved favorite location |

### Flags

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--json` | `-j` | `false` | Output in JSON format instead of human-readable |
| `--unit` | `-u` | `celsius` | Temperature unit: `celsius` or `fahrenheit` |

### Output (Human-Readable)

```
Weather for London, United Kingdom
──────────────────────────────────
Temperature:    15.3 °C
Conditions:     Partly cloudy
Humidity:       72%
Wind:           12.5 km/h NW
Pressure:       1013 hPa
Observed:       2026-05-11 14:30 UTC
```

### Output (JSON)

```json
{
  "location": {
    "name": "London",
    "country": "United Kingdom",
    "latitude": 51.5074,
    "longitude": -0.1278
  },
  "temperature": 15.3,
  "temperature_unit": "celsius",
  "conditions": "Partly cloudy",
  "humidity": 72.0,
  "wind_speed": 12.5,
  "wind_direction": "NW",
  "pressure": 1013.0,
  "timestamp": "2026-05-11T14:30:00Z"
}
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid arguments, no location specified) |
| `2` | Network error (connection failed, timeout) |
| `3` | API error (server returned error response) |
| `4` | Location not found (city name invalid) |

---

## Command: `weatherfrog forecast`

**Purpose**: Fetch weather forecast for a location.

### Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--city` | `-c` | Mutually exclusive with `--favorite` | City name to fetch forecast for |
| `--latitude` | `--lat` | Mutually exclusive with `--city` and `--favorite` | Geographic latitude |
| `--longitude` | `--lon` | Required with `--latitude` | Geographic longitude |
| `--favorite` | `-f` | Mutually exclusive with `--city` and `--latitude` | Name of a saved favorite location |
| `--days` | `-d` | `3` | Number of forecast days (1–16) |

### Flags

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--json` | `-j` | `false` | Output in JSON format instead of human-readable |
| `--unit` | `-u` | `celsius` | Temperature unit: `celsius` or `fahrenheit` |

### Output (Human-Readable)

```
Forecast for Berlin, Germany (3 days)
─────────────────────────────────────────────────────────────
Date         High    Low     Conditions        Humidity  Wind
─────────────────────────────────────────────────────────────
2026-05-11   18 °C   9 °C    Sunny             45%       8 km/h
2026-05-12   20 °C   11 °C   Partly cloudy     55%       12 km/h
2026-05-13   16 °C   8 °C    Rain              80%       20 km/h
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid arguments, no location specified) |
| `2` | Network error (connection failed, timeout) |
| `3` | API error (server returned error response) |
| `4` | Location not found (city name invalid) |
| `5` | Invalid days parameter (out of range 1–16) |

---

## Command: `weatherfrog favorites`

**Purpose**: Manage saved favorite locations.

### Subcommands

#### `weatherfrog favorites add`

Add a location to favorites.

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--city` | `-c` | Required | City name to add |

**Exit Codes**: `0` (success), `1` (invalid city), `6` (already exists)

#### `weatherfrog favorites list`

List all saved favorite locations.

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--json` | `-j` | `false` | Output in JSON format |

**Exit Codes**: `0` (success), `7` (no favorites saved)

#### `weatherfrog favorites remove`

Remove a location from favorites.

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--name` | `-n` | Required | Name of favorite to remove |

**Exit Codes**: `0` (success), `8` (favorite not found)

---

## Global Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--help` | `-h` | Display help information |
| `--version` | `-V` | Display application version |
| `--verbose` | `-v` | Enable verbose output (debug information) |

## Input Validation Rules

1. **Mutual exclusivity**: `--city`, `--latitude`/`--longitude`, and `--favorite` are mutually exclusive
2. **Coordinate pairing**: `--latitude` requires `--longitude` and vice versa
3. **Days range**: `--days` must be between 1 and 16 inclusive
4. **Unit values**: `--unit` must be `celsius` or `fahrenheit` (case-insensitive)
5. **City name**: Must be non-empty after trimming whitespace
6. **Favorite name**: Must match an existing favorite (case-insensitive)
