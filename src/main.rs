struct Container<T> {
    value: T
}

impl<T> Container<T> {
    fn new (value: T) -> Self {
        Container { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main () {
    let cont_1 = Container::new(10);
    let cont_2 = Container::new("hello");
    println!("{}", cont_1.get_value());
    println!("{}", cont_2.get_value());

}