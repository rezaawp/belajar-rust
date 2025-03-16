fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Reza Khoirul Wijaya Putra";
    println!("Hello {}", name);
}

#[test]
fn test_var_mutable() {
    let mut name = "Reza Khoirul";
    println!("Hello {}", name);
    
    name = "Abdul";
    println!("Hello {}", name);
}
