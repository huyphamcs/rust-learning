// src/lib.rs

// 1. Khai báo module `teacher` và làm cho nó public.
//    Rust sẽ tự động tìm file `src/teacher.rs`.
pub mod teacher;

// 2. Sử dụng `use` để tạo lối tắt đến struct `Teacher`.
use crate::teacher::Teacher;

// 3. Hàm public để bên ngoài có thể gọi.
pub fn class_session() {
    // Tạo một instance của Teacher.
    let teacher_1 = Teacher {
        name: String::from("Professor Dumbledore"),
        subject: String::from("Transfiguration"),
    };

    // Gọi hàm từ module `teacher`.
    teacher::grade_assignment(&teacher_1);
}