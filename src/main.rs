fn main() {
    let optional = Some(1);
    match optional {
        Some(i) => println!("{}", i),
        None => println!("No value."),
    }
}
