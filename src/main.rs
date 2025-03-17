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

#[test]
fn string() {
    let name: &str = "   Reza Khoirul Wijaya Putra    "; // ini akan terus ada di memory karna string slice bersifat fixed dan tidak akan bisa berubah
    println!("Name: {}", name); 
    let trim: &str = name.trim(); // ini akan menyimpan di stack memory baru
    println!("Name trim: {}", trim);

    let mut first_name = "Reza"; // ini akan ada terus di memory
    println!("First Name: {}", first_name);

    first_name = "Reza Khoirul"; // ini akan membuat data baru di memory stack tapi dengan catatan, value sebelumnya tidak akan hilang di memory

    println!("First Name: {}", first_name);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Reza Khoirul Wijaya Putra");

    let name2: String = name1;
    // println!("name1: {}, name2: {}", name1, name2); // name1 sudah gabisa lagi di akses disini karena sudah ada perpindahan ownersip dari name1 ke name2
    // perpindahan ownership akan terjadi jika yang disimpan di dalam heap

    println!("Name: {}", name2);
}

#[test]
fn clone() {
    let name1: String = String::from("Reza Khoirul Wijaya Putra");

    let name2: String = name1.clone();
    println!("name1: {}, name2: {}", name1, name2);
}

fn hello_world(name: String) {
    println!("name: {}", name);
}

#[test]
fn ownership_fn() {
    let name = String::from("Reza Khoirul Wijaya Putra");

    hello_world(name.clone()); // disini kalau gapakai clone, variable name sudah berpindah ownership nya ke parameter function hello_world yang menyebabkan variable yang disimpan di head tidak bisa di akses lagi setelah pemanggilan function ini

    println!("name in ownership_fn after pass data to hello_world function: {}", name);
}

fn full_name(first_name: String, last_name: String) -> (String, String) {
    println!("first_name in function full_name accesed: {}", first_name);
    println!("last_name in function full_name accesed: {}", last_name);
    return (first_name, last_name);
}

#[test]
// di bawah ini adalah cara mengembalikan ownership tanpa harus clone()
// di bawah ini adalah cara yang menyulitkan. karena rust sudah memiliki fitur Reference
fn test_full_name() {
    let first_name = String::from("Reza Khoirul");
    let last_name = String::from("Wijaya Putra");

    let (first_name, last_name) = full_name(first_name, last_name);

    println!("first_name accesed in test_full_name: {}", first_name);
    println!("last_name accesed in test_full_name: {}", last_name);
}

fn full_name_references(first_name: &String, last_name: &String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
// di bawah ini adalah cara mengembalikan ownership tanpa harus clone()
// di bawah ini adalah cara yang menyulitkan. karena rust sudah memiliki fitur Reference
fn test_preferences() {
    let first_name = String::from("Reza Khoirul");
    let last_name = String::from("Wijaya Putra");

    let fullname = full_name_references(&first_name, &last_name);

    println!("first_name accesed in test_full_name: {}", first_name);
    println!("last_name accesed in test_full_name: {}", last_name);
    println!("fullname result: {}", fullname);
}

fn change_value_preferences(full_name: &mut String) {
    full_name.push_str(" Khoirul");
}

#[test]
fn test_change_preferences() {
    let mut full_name = String::from("Reza");

    change_value_preferences(&mut full_name);

    println!("full_name: {}", full_name);
}