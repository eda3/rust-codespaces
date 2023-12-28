fn main() {
    let _s = concat!("A", "b2", 3);
    // _s = "Ab23"
    dbg!(_s);

    let _s = format!("{}-{:?}", _s, ("D", 5));
    // _s = "Ab23-(\"D\", 5)"
    dbg!(_s);

    let _s = format!("{}{}", "abc", "def");
    dbg!(_s);
    _s = "abcdef"
}
