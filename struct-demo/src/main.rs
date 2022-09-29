 #[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: &String, username: &String)-> User{
    let userinfo = User { username: username.to_string(), email: email.to_string(), sign_in_count: 1, active: true };
    println!("{:?}",userinfo);
    userinfo
}

fn main() {
    let user1 = User {
        email: String::from("jartto@qq.com"),
        username: String::from("jartto"),
        active: true,
        sign_in_count: 1
    };
    build_user(&user1.email,&user1.username);
}
