use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    // https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metrics&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
    > Temperature: {:.1}°C,
    > Humidity: {:.1}%,
    > Pressure: {:.1} hPa,
    > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather Station".bright_yellow());

    loop {
        println!("{}", "Please entery the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input!");

        let city = city.trim();

        println!(
            "{}",
            "Please enter the country code (e.g., US for United States):".bright_green()
        );

        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input"); // Reading user input for country code
        let country_code = country_code.trim();

        let api_key = "";

        match get_weather_info(city, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        ); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
