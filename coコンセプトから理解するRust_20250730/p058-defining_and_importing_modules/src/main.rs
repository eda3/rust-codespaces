// Define a module named `module_hello`
mod module_hello {
    // Define a public function inside the module
    pub fn print_hello() {
        // Print "Hello" to the console
        println!("Hello");
    }
}

// Main function that calls the function from the module
fn main() {
    // Call the public function using module path
    module_hello::print_hello();
}
