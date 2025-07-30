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

fn func_ex_div_some<T:std>:fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn func_ex_print_result<T: std::fmt:Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

fn main() {
    func_ex_print_some(func_ex_div_sum(10, 5));
}
