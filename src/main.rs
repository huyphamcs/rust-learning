fn main () {
    println!("{}", "Hello World");
    println!("{}", celsius_to_fahrenheit(42.9 as f32));
    println!("{}", fahrenheit_to_celsius(42.9 as f32));
    println!("{}", fibo(5));
    println!("{}", fibo(7));

}

fn celsius_to_fahrenheit (celsius: f32) -> f32 {
    celsius*(9 as f32) / (5 as f32) + (32 as f32)
}           

fn fahrenheit_to_celsius (fahrenheit: f32) -> f32 {
    (fahrenheit - (32 as f32))* (5 as f32) / (9 as f32)
}

fn fibo (n: usize) -> usize {
    if n == 1 || n == 2 {n-1}
    else {fibo(n-1) + fibo(n-2)}
}