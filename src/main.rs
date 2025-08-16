fn main() {
    use std::string::String;
    let str: &mut String = &mut String::from("Hello World");
    println!("{}", str);
    str.push_str("HAHAHA");
    println!("{}", calculate_length(str.to_string()));
    println!("{}", str);

}

fn calculate_length (str: String) -> usize {
    return str.len();
}