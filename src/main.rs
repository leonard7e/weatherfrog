mod api;
mod commands;
mod config;
mod error;
mod models;
mod output;
mod storage;

use clap::Parser;
use std::process;

use error::WeatherError;

#[derive(Parser)]
#[command(
    name = "weatherfrog",
    version,
    about = "A weather CLI using the Zero Cost Weather API"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short = 'v',
        long = "verbose",
        global = true,
        help = "Enable verbose output"
    )]
    verbose: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(about = "Fetch current weather for a location")]
    Fetch {
        #[arg(short = 'c', long = "city", help = "City name", conflicts_with_all = ["latitude", "favorite"])]
        city: Option<String>,

        #[arg(long = "latitude", help = "Geographic latitude", requires = "longitude", conflicts_with_all = ["city", "favorite"])]
        latitude: Option<f64>,

        #[arg(long = "longitude", help = "Geographic longitude")]
        longitude: Option<f64>,

        #[arg(short = 'f', long = "favorite", help = "Name of a saved favorite location", conflicts_with_all = ["city", "latitude"])]
        favorite: Option<String>,

        #[arg(short = 'j', long = "json", help = "Output in JSON format")]
        json: bool,

        #[arg(
            short = 'u',
            long = "unit",
            default_value = "celsius",
            help = "Temperature unit (celsius or fahrenheit)"
        )]
        unit: String,
    },

    #[command(about = "Fetch weather forecast for a location")]
    Forecast {
        #[arg(short = 'c', long = "city", help = "City name", conflicts_with_all = ["latitude", "favorite"])]
        city: Option<String>,

        #[arg(long = "latitude", help = "Geographic latitude", requires = "longitude", conflicts_with_all = ["city", "favorite"])]
        latitude: Option<f64>,

        #[arg(long = "longitude", help = "Geographic longitude")]
        longitude: Option<f64>,

        #[arg(short = 'f', long = "favorite", help = "Name of a saved favorite location", conflicts_with_all = ["city", "latitude"])]
        favorite: Option<String>,

        #[arg(
            short = 'd',
            long = "days",
            default_value = "3",
            help = "Number of forecast days (1–16)"
        )]
        days: u8,

        #[arg(short = 'j', long = "json", help = "Output in JSON format")]
        json: bool,

        #[arg(
            short = 'u',
            long = "unit",
            default_value = "celsius",
            help = "Temperature unit (celsius or fahrenheit)"
        )]
        unit: String,
    },

    #[command(about = "Manage saved favorite locations")]
    Favorites {
        #[command(subcommand)]
        action: FavoritesAction,
    },
}

#[derive(clap::Subcommand)]
enum FavoritesAction {
    #[command(about = "Add a location to favorites")]
    Add {
        #[arg(short = 'c', long = "city", required = true, help = "City name to add")]
        city: String,
    },
    #[command(about = "List all saved favorite locations")]
    List {
        #[arg(short = 'j', long = "json", help = "Output in JSON format")]
        json: bool,
    },
    #[command(about = "Remove a location from favorites")]
    Remove {
        #[arg(
            short = 'n',
            long = "name",
            required = true,
            help = "Name of favorite to remove"
        )]
        name: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let api_client = api::ApiClient::new();

    let result: Result<(), WeatherError> = match &cli.command {
        Commands::Fetch {
            city,
            latitude,
            longitude,
            favorite,
            json,
            unit,
        } => {
            if cli.verbose {
                eprintln!("[verbose] Fetching current weather...");
            }
            handle_fetch(
                &api_client,
                city.as_deref(),
                *latitude,
                *longitude,
                favorite.as_deref(),
                *json,
                unit,
            )
            .await
        }
        Commands::Forecast {
            city,
            latitude,
            longitude,
            favorite,
            days,
            json,
            unit,
        } => {
            if cli.verbose {
                eprintln!("[verbose] Fetching forecast...");
            }
            handle_forecast(
                &api_client,
                city.as_deref(),
                *latitude,
                *longitude,
                favorite.as_deref(),
                *days,
                *json,
                unit,
            )
            .await
        }
        Commands::Favorites { action } => {
            if cli.verbose {
                eprintln!("[verbose] Managing favorites...");
            }
            handle_favorites(&api_client, action).await
        }
    };

    if let Err(e) = &result {
        if cli.verbose {
            eprintln!("[verbose] Error details: {:?}", e);
        }
        eprintln!("Error: {}", e);
        let exit_code = exit_code_for_error(e);
        process::exit(exit_code);
    }
}

async fn handle_fetch(
    client: &api::ApiClient,
    city: Option<&str>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    favorite: Option<&str>,
    json: bool,
    unit: &str,
) -> Result<(), WeatherError> {
    let location = resolve_location(client, city, latitude, longitude, favorite).await?;
    let mut weather = client.fetch_current(&location).await?;
    weather.location = location;
    weather.temperature_unit = parse_unit(unit);

    if json {
        let out = output::format_weather_json(&weather)?;
        println!("{}", out);
    } else {
        println!("{}", output::format_weather_text(&weather));
    }
    Ok(())
}

async fn handle_forecast(
    client: &api::ApiClient,
    city: Option<&str>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    favorite: Option<&str>,
    days: u8,
    json: bool,
    unit: &str,
) -> Result<(), WeatherError> {
    if !(config::MIN_FORECAST_DAYS..=config::MAX_FORECAST_DAYS).contains(&days) {
        return Err(WeatherError::Validation {
            field: "days".to_string(),
            constraint: format!(
                "Days must be between {} and {}",
                config::MIN_FORECAST_DAYS,
                config::MAX_FORECAST_DAYS
            ),
        });
    }

    let location = resolve_location(client, city, latitude, longitude, favorite).await?;
    let mut forecast = client.fetch_forecast(&location, days).await?;
    forecast.location = location;
    forecast.temperature_unit = parse_unit(unit);

    if json {
        let out = output::format_forecast_json(&forecast)?;
        println!("{}", out);
    } else {
        println!("{}", output::format_forecast_text(&forecast));
    }
    Ok(())
}

async fn handle_favorites(
    client: &api::ApiClient,
    action: &FavoritesAction,
) -> Result<(), WeatherError> {
    match action {
        FavoritesAction::Add { city } => {
            let prefs = storage::load_preferences().unwrap_or_default();
            let results = client.geocode(city).await?;
            let result = results.first().ok_or_else(|| WeatherError::Validation {
                field: "city".to_string(),
                constraint: format!("City '{}' not found", city),
            })?;
            let location = models::Location::new(
                result.name.clone(),
                result.latitude,
                result.longitude,
                result.country.clone(),
            )?;
            if prefs
                .favorites
                .iter()
                .any(|f| f.name.to_lowercase() == location.name.to_lowercase())
            {
                return Err(WeatherError::Validation {
                    field: "favorite".to_string(),
                    constraint: format!("'{}' is already a favorite", location.name),
                });
            }
            let mut new_prefs = prefs;
            new_prefs.favorites.push(location.clone());
            storage::save_preferences(&new_prefs)?;
            println!("Added '{}' to favorites", location.name);
            Ok(())
        }
        FavoritesAction::List { json } => {
            let prefs = storage::load_preferences().unwrap_or_default();
            if prefs.favorites.is_empty() {
                return Err(WeatherError::Validation {
                    field: "favorite".to_string(),
                    constraint: "No favorites saved. Use 'weatherfrog favorites add --city <name>' to add one."
                        .to_string(),
                });
            }
            if *json {
                let out = serde_json::to_string_pretty(&prefs.favorites).map_err(|e| {
                    WeatherError::Parse {
                        message: format!("Failed to serialize favorites: {}", e),
                        field: "favorites_json".to_string(),
                    }
                })?;
                println!("{}", out);
            } else {
                println!("Saved favorites:");
                for (i, fav) in prefs.favorites.iter().enumerate() {
                    let country = fav.country.as_deref().unwrap_or("");
                    if country.is_empty() {
                        println!("  {}. {}", i + 1, fav.name);
                    } else {
                        println!("  {}. {}, {}", i + 1, fav.name, country);
                    }
                }
            }
            Ok(())
        }
        FavoritesAction::Remove { name } => {
            let prefs = storage::load_preferences().unwrap_or_default();
            let index = prefs
                .favorites
                .iter()
                .position(|f| f.name.to_lowercase() == name.to_lowercase());
            match index {
                Some(idx) => {
                    let mut new_prefs = prefs;
                    let removed = new_prefs.favorites.remove(idx);
                    storage::save_preferences(&new_prefs)?;
                    println!("Removed '{}' from favorites", removed.name);
                    Ok(())
                }
                None => Err(WeatherError::Validation {
                    field: "name".to_string(),
                    constraint: format!("Favorite '{}' not found", name),
                }),
            }
        }
    }
}

async fn resolve_location(
    client: &api::ApiClient,
    city: Option<&str>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    favorite: Option<&str>,
) -> Result<models::Location, WeatherError> {
    if let Some(city_name) = city {
        let results = client.geocode(city_name).await?;
        let result = results.first().ok_or_else(|| WeatherError::Validation {
            field: "city".to_string(),
            constraint: format!("City '{}' not found", city_name),
        })?;
        models::Location::new(
            result.name.clone(),
            result.latitude,
            result.longitude,
            result.country.clone(),
        )
    } else if let Some(fav_name) = favorite {
        let prefs = storage::load_preferences().map_err(|_| WeatherError::Validation {
            field: "favorite".to_string(),
            constraint: "No favorites saved".to_string(),
        })?;
        prefs
            .favorites
            .into_iter()
            .find(|f| f.name.to_lowercase() == fav_name.to_lowercase())
            .ok_or_else(|| WeatherError::Validation {
                field: "name".to_string(),
                constraint: format!("Favorite '{}' not found", fav_name),
            })
    } else if let (Some(lat), Some(lon)) = (latitude, longitude) {
        models::Location::new(String::new(), lat, lon, None)
    } else {
        Err(WeatherError::Validation {
            field: "location".to_string(),
            constraint:
                "Provide a city name (--city), coordinates (--latitude/--longitude), or a favorite (--favorite)"
                    .to_string(),
        })
    }
}

fn parse_unit(s: &str) -> models::TemperatureUnit {
    match s.to_lowercase().as_str() {
        "fahrenheit" | "f" => models::TemperatureUnit::Fahrenheit,
        _ => models::TemperatureUnit::Celsius,
    }
}

fn exit_code_for_error(e: &WeatherError) -> i32 {
    match e {
        WeatherError::Network { .. } => config::EXIT_NETWORK_ERROR,
        WeatherError::Api { .. } => config::EXIT_API_ERROR,
        WeatherError::Parse { .. } => config::EXIT_GENERAL_ERROR,
        WeatherError::Validation { field, .. } => match field.as_str() {
            "city" | "location" | "name" => config::EXIT_LOCATION_NOT_FOUND,
            "days" => config::EXIT_INVALID_PARAMETER,
            "favorite" => config::EXIT_DUPLICATE_FAVORITE,
            _ => config::EXIT_GENERAL_ERROR,
        },
        WeatherError::Storage { .. } => config::EXIT_GENERAL_ERROR,
        WeatherError::Config { .. } => config::EXIT_GENERAL_ERROR,
    }
}
