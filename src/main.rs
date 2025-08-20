fn main() {
    let mut s = "first";
    println!("{}", to_pig_latin(&s))
}

fn to_pig_latin(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let mut chars = word.chars();
        let first_char = match chars.next() {
            Some(c) => c,
            None => continue, // Bỏ qua từ rỗng
        };

        let pig_word = match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                format!("{}-hay", word)
            }
            _ => {
                format!("{}-{}ay", chars.as_str(), first_char)
            }
        };
        result.push_str(&pig_word);
        result.push(' ');
    }
    result.trim().to_string() // Xóa dấu cách thừa ở cuối
}