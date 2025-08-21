// src/lib.rs

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
 
// Bắt đầu một module test
#[cfg(test)]
mod tests {
    // Import mọi thứ từ module cha (bên ngoài)
    use super::*;

    // Đánh dấu đây là một hàm test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // Kiểm tra xem `result` có bằng 4 không
        // Nếu không bằng, test sẽ fail và panic
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // Bạn có thể dùng `assert!` cho các biểu thức boolean
        assert!(1 + 1 == 2, "Giá trị không bằng 2");
    }

    #[test]
    #[should_panic] // Đánh dấu test này được kỳ vọng sẽ panic
    fn it_panics() {
        // Giả sử có một hàm sẽ panic với đầu vào không hợp lệ
        // greeting("world"); // giả định hàm này sẽ panic
        panic!("This test is supposed to panic");
    }
}