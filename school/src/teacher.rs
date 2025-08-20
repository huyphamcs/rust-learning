// src/teacher.rs

// Struct `Teacher` được đánh dấu `pub` để bên ngoài có thể truy cập.
pub struct Teacher {
    // Các trường của struct cũng cần `pub` nếu muốn truy cập trực tiếp.
    pub name: String,
    pub subject: String,
}

// Hàm này cũng phải là `pub` để có thể gọi từ `lib.rs`.
pub fn grade_assignment(teacher: &Teacher) {
    println!("{} is grading an assignment.", teacher.name);
}