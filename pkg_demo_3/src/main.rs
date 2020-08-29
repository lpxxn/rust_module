use utils::User;

fn main() {
    let u = User::new_user(String::from("tom"), 5);
    println!("user: {:#?}", u);
}
