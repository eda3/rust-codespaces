// A simple function that adds two i32 integers.
fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

// Test case 1: 1 + 2 = 3 -> should pass.
#[test]
fn test1() {
    assert_eq!(add_i32(1, 2), 3); // Success
}

// Test case 2: 2 + 4 = 6, but expected 7 => should fail
#[test]
fn test2() {
    assert_eq!(add_i32(2, 4), 7) // Failure
}

// Main function for manual run: prints 2 + 5 = 7
fn main() {
    println!("{}", add_i32(2, 5));
}
