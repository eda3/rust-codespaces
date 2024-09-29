fn return_hello() -> &String {
    let s = "Hello".to_string();
    &s
}
fn main() {
    let s = return_hello();
    println!("{}", s);
}
