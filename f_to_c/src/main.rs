fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let fahrenheit_temp = 68.0;
    let celsius_temp_1 = 20.0;
    let celsius_temp_2 = 200.0;

    let converted_to_celsius = fahrenheit_to_celsius(fahrenheit_temp);
    let converted_to_fahrenheit_1 = celsius_to_fahrenheit(celsius_temp_1);
    let converted_to_fahrenheit_2 = celsius_to_fahrenheit(celsius_temp_2);

    println!("{}°F is equal to {:.2}°C", fahrenheit_temp, converted_to_celsius);
    println!("{}°C is equal to {:.2}°F", celsius_temp_1, converted_to_fahrenheit_1);
    println!("{}°C is equal to {:.2}°F", celsius_temp_2, converted_to_fahrenheit_2);
}