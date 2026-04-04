fn main() {
    println!("Hello, world!");

    let p1 = Person {
        name: String::from("Landry"),
        age: 20
    };


}

struct Person {
    name: String,
    age: u32,
}
