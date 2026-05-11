# Research: WeatherFrog CLI

**Date**: 2026-05-11  
**Feature**: 001-weather-cli

## Technical Decisions

### Decision: HTTP Client Library
**Choice**: `reqwest` with `tokio` async runtime

**Rationale**: `reqwest` is the most widely used HTTP client in the Rust ecosystem. It provides native async support, automatic connection pooling, and integrates well with `serde` for JSON deserialization. The `tokio` runtime is the de facto standard for async Rust and provides reliable cross-platform support.

**Alternatives considered**:
- `ureq` - Synchronous, simpler, but lacks async support and connection pooling
- `curl` (via `curl-rust`) - Mature but requires libcurl dependency, more complex FFI
- `isahc` - Good alternative but smaller ecosystem than reqwest

### Decision: CLI Framework
**Choice**: `clap` v4 with derive macros

**Rationale**: `clap` v4 with derive macros provides a clean, declarative way to define subcommands and arguments. It automatically generates `--help` output, supports shell completion generation, and handles argument validation. The derive API reduces boilerplate compared to the builder API.

**Alternatives considered**:
- `structopt` - Deprecated in favor of clap v4 derive macros
- `clap` builder API - More verbose, less ergonomic for subcommand patterns
- `bpaf` - Lightweight alternative but smaller ecosystem and fewer features

### Decision: Error Handling Strategy
**Choice**: Custom `WeatherError` enum with `thiserror` crate

**Rationale**: `thiserror` provides a derive macro for `std::error::Error` that generates clean error types from enum variants. Combined with the `?` operator, it enables idiomatic error propagation without `.unwrap()` or `.expect()`. Each error variant maps to a specific failure mode (API, network, validation, storage).

**Alternatives considered**:
- `anyhow` - Good for applications but loses type information; better for top-level error handling
- `Box<dyn Error>` - Loses type information, harder to match on specific errors
- Manual `Error` impl - More boilerplate, no derive convenience

### Decision: JSON Serialization
**Choice**: `serde` with `serde_json`

**Rationale**: `serde` is the standard serialization framework in Rust. Combined with `serde_json`, it provides seamless conversion between Rust structs and JSON. The derive macros (`Serialize`, `Deserialize`) reduce boilerplate and handle most common patterns automatically.

**Alternatives considered**:
- `serde_yaml` - Not needed; JSON is the primary format
- Manual JSON construction - Error-prone, no type safety

### Decision: Async vs Sync
**Choice**: Async with `tokio` runtime

**Rationale**: HTTP requests are inherently I/O-bound. Using async Rust with `reqwest` provides better resource utilization and enables future extensibility (e.g., concurrent requests for multiple locations). The `tokio` runtime handles the async execution transparently.

**Alternatives considered**:
- Synchronous with `ureq` - Simpler but blocks the thread during network I/O
- `smol` runtime - Smaller footprint but less ecosystem support than tokio

### Decision: Configuration Storage
**Choice**: JSON file at `~/.weatherfrog/favorites.json`

**Rationale**: A simple JSON file is easy to debug, version-control, and doesn't require additional dependencies. The `dirs` crate provides cross-platform home directory resolution. JSON format aligns with the application's existing serde usage.

**Alternatives considered**:
- SQLite - Overkill for simple favorites storage
- TOML - Better for config files, but JSON is more natural for list data
- OS-specific preferences (AppData/Preferences) - More complex, platform-specific

### Decision: API Endpoint
**Choice**: Zero Cost Weather API (open-meteo.com)

**Rationale**: Open-Meteo is a free, open-source weather API that requires no authentication, provides current weather and forecast data, and has a well-documented REST interface. It supports geocoding for city name lookups and returns JSON responses.

**Alternatives considered**:
- WeatherAPI.com - Requires API key, limited free tier
- OpenWeatherMap - Requires API key, rate-limited free tier
- wttr.in - Text-based output, harder to parse programmatically

## API Integration Patterns

### Request Flow
1. Parse user input (city name or coordinates)
2. If city name: geocode to coordinates via API
3. Fetch weather/forecast data using coordinates
4. Parse JSON response into domain models
5. Format and display output

### Error Handling Flow
1. Network errors → `WeatherError::Network` with descriptive message
2. API errors (non-200) → `WeatherError::Api` with status code and message
3. Parse errors → `WeatherError::Parse` with field details
4. Validation errors → `WeatherError::Validation` with constraint details
5. Storage errors → `WeatherError::Storage` with file path details

### Rate Limiting
- Open-Meteo has no explicit rate limits for reasonable usage
- Implement exponential backoff (1s, 2s, 4s, max 3 retries) for transient failures
- Display retry-after information to user on rate limit responses

## Best Practices

### Rust-Specific
- Use `Result<T, WeatherError>` for all fallible operations
- Use `Option<T>` for nullable fields
- Propagate errors with `?` operator throughout
- Never use `.unwrap()` or `.expect()` in production code
- Use iterators and combinators (`map`, `filter`, `fold`) instead of manual loops
- Avoid nested `match` expressions; use `if let`, `while let`, and combinators
- Document all public items with `///` doc comments
- Use `#[derive(Debug)]` on all error types for better error messages

### Testing
- Unit tests for model validation, output formatting, and error handling
- Integration tests for API interaction (mock HTTP responses)
- CLI tests using `assert_cmd` crate for end-to-end verification
- Test error paths explicitly (network failure, parse errors, missing data)
