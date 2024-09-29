fn main() {
    let x;
    {
        let y = 1;
        x = &y;
    }
    println!("{}", x)
}
