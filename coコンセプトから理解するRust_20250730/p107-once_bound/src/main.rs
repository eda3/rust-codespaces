fn main() {
    // Create a new String object from a string literal
    let s = "Hello".to_string();

    // Move ownership of the String from `s` to `to`
    let t = s;

    // This works: `t` now owns the String
    println!("{t}");

    // This fails: `s` no longer owns the value (it's been moved)
    // println!("{s}");
}
