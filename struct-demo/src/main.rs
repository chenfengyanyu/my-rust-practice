struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String)-> User{
    User { username, email, sign_in_count: 1, active: ture }
}

fn main() {
    let mut user1 = User {
        email: String::from("jartto@qq.com"),
        username: String::from("jartto"),
        active: true,
        sign_in_count: 1
    };
    build_user(user1.email,user1.username);
    
}
