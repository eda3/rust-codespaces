// Define a function that takes a slice of i32s and returns a subslice
fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn main() {
    // Declare a fixed-size array of integers
    let v1 = [1, 2, 3, 4, 5];

    // Call the funtion with a reference to the array and an index value
    let p = pick1(&v1, 2); // 'p' now refers to the first two elements: &[1, 2]

    // Iterate over the returned slice and print each element
    for ss in p {
        println!("{ss}");
    }
}
