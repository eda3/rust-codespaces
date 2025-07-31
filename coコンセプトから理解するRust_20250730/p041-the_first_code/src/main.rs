// Divide x by y.
// If y is 0, return "None" (no value); otherwise return the result inside Some.
fn func_ex_div_sum(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

// Divide x by y.
// If y is 0, return an error message; otherwise return the result.
fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("divide by zero")
    } else {
        Ok(x / y)
    }
}

// Print the value inside an Option.
// If it is Some, print the number; if it is None, print "None".
fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{x}");
    } else {
        println!("None");
    }
}

// Same as adove, but uses a match statement.
fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{x}"),
        None => println!("None"),
    }
}

// Print the value inside a Result.
// If it is Ok, print the result; if it is err, print the error text.
fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{res}"),
        Err(str) => println!("{str}"),
    }
}

fn main() {
    func_ex_print_some(func_ex_div_sum(10, 5));
    func_ex_print_some(func_ex_div_sum(10, 0));
    func_ex_print_some_match(func_ex_div_sum(10, 5));
    func_ex_print_some_match(func_ex_div_sum(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
}
