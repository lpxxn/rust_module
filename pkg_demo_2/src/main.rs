use my_rust_lib_1;
fn main() {
    println!("Hello, world!");
    println!("{}", my_rust_lib_1::add(1, 2));
    let u = my_rust_lib_1::User::new_user(String::from("tom"), 2);
    println!("user: {:#?}", u);
}
