// src/main.rs
use std::env; // Module để làm việc với môi trường
use std::fs;   // Module để làm việc với file system

fn main() {
    // 1. Đọc các đối số dòng lệnh
    // env::args() trả về một iterator
    // .collect() biến iterator thành một collection, ở đây là Vector
    let args: Vec<String> = env::args().collect();

    // 2. Lấy query và tên file từ vector
    // args[0] luôn là tên của chương trình
    let query = &args[1];
    let file_path = &args[2];   

    println!("Searching for '{}'", query);
    println!("In file: {}", file_path);

    // 3. Đọc nội dung file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}