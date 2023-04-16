struct Person {
    name: String,
    age: i8,
    hobby: String,
}

impl Person {
    fn new(name: String, age: i8, hobby: String) -> Self {
        Person { name, age, hobby }
    }
}

fn main() {
    let bob = Person::new("ボブ".to_string(), 23, "ヒップホップ".to_string());
    println!("{}は{}歳で趣味は{}です。", bob.name, bob.age, bob.hobby);
}
