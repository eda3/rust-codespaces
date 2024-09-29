fn main() {
    let mut x = 1;
    let x_ref = &x;

    x = 2;
    println!("{}", x_ref);
}
