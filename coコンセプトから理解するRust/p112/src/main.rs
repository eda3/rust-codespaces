fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    let s_ref = &mut s;

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s={}", s);

    let s_ref2 = &mut s;
    myclear(s_ref2);
    println!("s={}", s);
}
