fn func_ex_div_sum(x:i32, y:i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    ans
}


fn main() {
    println!("Hello, world!");
}
