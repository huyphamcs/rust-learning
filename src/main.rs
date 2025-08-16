use rand::Rng;
use std::io; // Import the Rng trait
fn main() {
    let random_number = generate_random_number();
    // let guess = string_to_usize(get_user_input());

    loop {
        let guess = string_to_usize(get_user_input());
        if guess > random_number {
            println!("Lower");
        }
        else if guess < random_number {println!("Higher")}
        else {break;}
    }
    println!("Correct");

}

fn generate_random_number() -> usize {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    // Generate a random number within a specific range (e.g., 0 to 99)
    let n: usize = rng.gen_range(0..100);
    // println!("Random number (0-99): {}", n);
    return n;
}

fn string_to_usize(str: String) -> usize {
    return str.parse().expect("Failed to parse");
}

fn get_user_input() -> String {
    // println!("Please enter some text:"); // Prompt the user

    let mut input_string = String::new(); // Create a new, mutable String to store the input
    // let output: String;
    io::stdin() // Get the standard input handle
        .read_line(&mut input_string) // Read a line from stdin into the string
        .expect("Failed to read line"); // Handle potential errors during input

    // println!("You entered: {}", input_string.trim());
    return String::from(input_string.trim());
}
