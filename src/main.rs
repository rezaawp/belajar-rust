fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name: &str = "Reza Khoirul Wijaya Putra";
    println!("Hello {}", name);
}

#[test]
fn test_var_mutable() {
    let mut name: &str = "Reza Khoirul";
    println!("Hello {}", name);
    
    name = "Abdul";
    println!("Hello {}", name);
}

#[test]
fn test_static_typing() {
    // tipe data akan di deklarasikan di awal

    // contoh error (tidak sesuai tipe data)
    // let mut name = "Reza";
    // name = 10;


}

#[test]
fn test_shadowing() {
    let name: &str = "rezawp";
    println!("Hello {}", name);

    let name: i32 = 10;
    println!("Hello {}", name);
}

#[test]
fn test_data_type() {
    let lists: [i32; 2] = [1,2];

    println!("lists {:?}", lists); 
}