# Quickstart: WeatherFrog CLI

**Date**: 2026-05-11  
**Feature**: 001-weather-cli

## Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Internet connection for API requests

## Setup

```bash
# Clone the repository
git clone <repository-url>
cd weatherfrog

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage

### Fetch Current Weather

```bash
# By city name
./target/release/weatherfrog fetch --city "London"

# By coordinates
./target/release/weatherfrog fetch --latitude 51.5074 --longitude -0.1278

# JSON output
./target/release/weatherfrog fetch --city "Berlin" --json

# Fahrenheit
./target/release/weatherfrog fetch --city "New York" --unit fahrenheit
```

### Fetch Forecast

```bash
# Default 3-day forecast
./target/release/weatherfrog forecast --city "Paris"

# 7-day forecast
./target/release/weatherfrog forecast --city "Tokyo" --days 7

# JSON output
./target/release/weatherfrog forecast --city "Sydney" --json
```

### Manage Favorites

```bash
# Add a favorite location
./target/release/weatherfrog favorites add --city "London"

# List favorites
./target/release/weatherfrog favorites list

# Remove a favorite
./target/release/weatherfrog favorites remove --name "London"

# Fetch weather for a favorite
./target/release/weatherfrog fetch --favorite "London"
```

### Help

```bash
# General help
./target/release/weatherfrog --help

# Subcommand help
./target/release/weatherfrog fetch --help
./target/release/weatherfrog forecast --help
./target/release/weatherfrog favorites --help
```

## Project Structure

```
src/
├── main.rs              # CLI entry point
├── error.rs             # Custom error types
├── config.rs            # Configuration constants
├── api/                 # API client module
├── models/              # Domain data models
├── commands/            # Command handlers
├── storage/             # Local file storage
└── output/              # Output formatting
```

## Development

```bash
# Run in debug mode
cargo run -- fetch --city "London"

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Error Handling

All errors are handled gracefully without panics. Common error scenarios:

- **Network failure**: Clear message with suggestion to check connection
- **Invalid city**: Message suggesting to verify the city name
- **API error**: Status code and error message from API
- **Missing favorite**: List of available favorites

Exit codes follow POSIX conventions:
- `0` = Success
- `1` = General error
- `2` = Network error
- `3` = API error
- `4` = Location not found
- `5` = Invalid parameter
- `6` = Duplicate favorite
- `7` = No favorites
- `8` = Favorite not found
