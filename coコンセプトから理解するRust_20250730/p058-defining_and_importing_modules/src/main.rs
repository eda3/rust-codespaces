mod module_hello {
    pub fn print_hello() {
        println!("Hello");
    }
}

fn main() {
    module_hello::print_hello();
}
