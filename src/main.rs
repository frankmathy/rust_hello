// Tutorial: https://www.youtube.com/watch?v=VBdgdhhb3OE

mod env;
use reqwest::blocking::get;
use serde_json::{ Value, from_str};
use std::io;

fn main() {
    println!("Herzlich willkommen! In diesem Programm kannst du dir das Wetter anzeigen lassen.");
    let mut city: String = String::new();
    println!("Bitte gib eine Stadt ein:");
    io::stdin().read_line(&mut city).expect("Fehler beim Lesen der Eingabe");
    let api_key = env::API_KEY;
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    let response = get(&url).unwrap();
    let respose_text = response.text().unwrap();
    let json: Value = from_str(&respose_text).expect("JSON was not well formatted");
    let temp_kelvin: f64 = json["main"]["temp"].as_f64().unwrap();
    let temp_celsius: f64 =  temp_kelvin - 273.15;

    println!("Hier ist das Wetter in {}", city);
    println!("Aktuell {} Grad Celsius", temp_celsius.round());
    if temp_celsius > 20.0 {
        println!("Wow, ganz schön heiß heute!");
    } else {
        println!("Puh, hier ist es aber kalt!");
    }
    println!("\n\n");
}
