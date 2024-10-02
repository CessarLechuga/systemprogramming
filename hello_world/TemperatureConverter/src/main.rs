const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    let mut temperature_f = 32.0;
    
    println!("{}°F is equal to {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));
    
    for _ in 0..5 {
        temperature_f += 1.0;
        println!("{}°F is equal to {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));
    }
}