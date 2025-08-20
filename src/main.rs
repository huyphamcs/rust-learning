fn main () {
    println!("{}", "Hello World");
    println!("{}", celsius_to_fahrenheit(42.9 as f32));
    println!("{}", fahrenheit_to_celsius(42.9 as f32));

}

fn celsius_to_fahrenheit (celsius: f32) -> f32 {
    celsius*(9 as f32) / (5 as f32) + (32 as f32)
}           

fn fahrenheit_to_celsius (fahrenheit: f32) -> f32 {
    (fahrenheit - (32 as f32))* (5 as f32) / (9 as f32)
}