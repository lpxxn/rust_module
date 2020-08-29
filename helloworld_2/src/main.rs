mod user_info;
use user_info::user::User;

fn main() {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
}
