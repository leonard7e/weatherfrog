# WeatherFrog 🐸

A fast, zero-cost weather CLI tool written in Rust.

## Features

- **Current Weather**: Get real-time weather data for any city.
- **Forecast**: Fetch up to 16-day forecasts.
- **Favorites**: Save and manage your most-used locations.
- **JSON Support**: Machine-readable output for scripts and integration.
- **Cross-Platform**: Works on Linux, macOS, and Windows.

## Installation

```bash
cargo install --path .
```

## CLI Reference

### Main Commands
| Command | Description |
|:--- |:--- |
| `fetch` | Get current weather conditions |
| `forecast` | Get weather forecast for upcoming days |
| `favorites` | Manage saved locations (add, list, remove) |

### Common Options
| Option | Short | Description | Default |
|:--- |:--- |:--- |:--- |
| `--city` | `-c` | Name of the city | - |
| `--latitude` | - | Geographic latitude | - |
| `--longitude` | - | Geographic longitude | - |
| `--favorite` | `-f` | Name of a saved favorite | - |
| `--unit` | `-u` | Temperature unit (`celsius`, `fahrenheit`) | `celsius` |
| `--json` | `-j` | Output in JSON format | `false` |
| `--verbose` | `-v` | Enable verbose logging | `false` |

## Quick Start

### Fetch Weather
```bash
# By city name
weatherfrog fetch --city "Berlin"

# By coordinates
weatherfrog fetch --latitude 52.52 --longitude 13.405

# 7-day forecast
weatherfrog forecast --city "Tokyo" --days 7

# Using Fahrenheit and JSON output
weatherfrog fetch --city "New York" --unit fahrenheit --json
```

### Manage Favorites
```bash
# Add a favorite
weatherfrog favorites add --city "London"

# List saved favorites
weatherfrog favorites list

# Fetch weather using a favorite
weatherfrog fetch --favorite "London"

# Remove a favorite
weatherfrog favorites remove --name "London"
```

## License
Apache-2.0
