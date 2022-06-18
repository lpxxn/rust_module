mod user_info;
use user_info::user::User;

mod student;
use student::user::User as User2;

fn main() {
    let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", user_info::user::add(1, 2));
    
    println!("{:?}", User2::new_user("hello".to_string(), 111));

    student::abc::hello();
}
