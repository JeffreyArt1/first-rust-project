struct Person {
    name: String,
    age: i32,
}

fn main() {
    let jeffrey = Person {
        name: String::from("Jeffrey Mesa"),
        age: 24,
    };

    println!(
        "Hola, me llamo {} y tengo {} años 🥸",
        jeffrey.name, jeffrey.age
    )
}
