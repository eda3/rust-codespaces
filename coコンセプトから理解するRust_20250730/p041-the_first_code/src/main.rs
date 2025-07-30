fn func_ex_div_sum(x:i32, y:i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    ans
}

fn func_ex_div_result(x:i32, y:i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("divide by zero")
    } else {
        Ok(x / y)
    }
}


fn main() {
    println!("Hello, world!");
}
