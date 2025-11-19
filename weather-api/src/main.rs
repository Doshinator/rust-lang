use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OpenMetoResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    weather_code: i32,
}

// we'll send this to our users (cleaner format)
#[derive(Serialize, Debug)]
struct WeatherReponse {
    city: String,
    temperature_celsius: f64,
    condition: String, 
}

fn weather_code_to_condition(code: i32) -> String {
    match code {
        0 => "Clear sky".to_string(),
        1 | 2 | 3 => "Partly cloudy".to_string(),
        45 | 48 => "Foggy".to_string(),
        51 | 53 | 55 => "Drizzle".to_string(),
        61 | 63 | 65 => "Rain".to_string(),
        71 | 73 | 75 => "Snow".to_string(),
        95 => "Thunderstorm".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn get_city_coordinates(city: &str) -> Option<(f64, f64)> {
    match city.to_lowercase().as_str() {
        "newyork" | "nyc" => Some((40.7128, -74.0060)),
        "london" => Some((51.5074, -0.1278)),
        "tokyo" => Some((35.6762, 139.6503)),
        "sydney" => Some((-33.8688, 151.2093)),
        "paris" => Some((48.8566, 2.3522)),
        _ => None,
    }
}

async fn get_weather(city: web::Path<String>) -> impl Responder {
    let cord = match get_city_coordinates(&city) {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("City '{}' not found. Try: newyork, london, tokyo, sydney, paris", city)
            })));
        },
    };

    // API URL
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code",
        cord.0,
        cord.1,
    );
    
    // Client call
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await;

    match response {
        Ok(resp) => todo!(),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch weather data"
        }))),
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting weather API on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0", 8080))?
    .run()
    .await
}
