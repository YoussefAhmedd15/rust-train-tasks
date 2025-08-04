#[deny(clippy::all)]

// Task 1
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");
//     let answer = concatenate_strings(&s1, &s2);
//     println!("{answer}")
// }
// fn concatenate_strings(s1: &str, s2: &str) -> String {
//     let s3: String = format!("{}{}", s1, s2);
//     s3
// }

// Task 2
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let person = Person {
        name: "john".to_string(),
        age: 19,
    };
}
