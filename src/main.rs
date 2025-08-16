fn main() {
    println!("{}", add(1,2));
    println!("Factorial: {}", factorial(2))
}


fn add (a: u64, b: u64)-> u64 {
    return a+b;
}



fn factorial (n: u64) -> u64{
    if n<=2 {
        return n;
    }
    return n*factorial(n-1);
}