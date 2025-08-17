fn main () {
    print_numbers(10);
}
fn print_numbers(n: usize) {
    
    // Solution 1
    // let mut i = 0;
    // loop {
    //     println!("{i}");
    //     i += 1;
    //     if i == n {break;}
    // }
    // Solution 2
    // let mut i = 0;
    // while i<n {
    //     println!("{i}");
    //     i += 1;
    // }
    // Solution 3
    for i in 0..10 {
        println!("{i}");
    }
}