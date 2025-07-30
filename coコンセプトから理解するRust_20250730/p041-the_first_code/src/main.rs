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

fn func_ex_div_some<T:std:fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn main() {
    println!("Hello, world!");
}
