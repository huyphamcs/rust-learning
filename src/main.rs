#[derive(Debug)]

struct Student {
    name: String,
    age: usize,
    email: String,
    active: bool,
}

fn main() {
    let first_student: Student = Student {
        name: String::from("Pham Quoc Huy"),
        age: 19,
        email: String::from("a@gmail.com"),
        active: false,
    };
    let second_student = create_member(
        first_student.name.clone(),
        first_student.age,
        first_student.email.clone(),
        first_student.active,
    );
    println!("{:#?}", first_student);

    println!("{:#?}", second_student);
}

fn create_member(name: String, age: usize, email: String, active: bool) -> Student {
    return Student {
        name,
        age,
        email,
        active,
    };
}
