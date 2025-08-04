#[deny(clippy::all)]
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let answer = concatenate_strings(&s1, &s2);
    println!("{answer}")
}
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let s3: String = format!("{}{}", s1, s2);
    s3
}
