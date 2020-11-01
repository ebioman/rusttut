struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let mut user1 = User{
        email: String::from("Emanuel.schmid@gmail.com"),
        username: String::from("eschmid"),
        active: true,
        sign_in_count: 33,
    };
    user1.email= String::from("newEmail@test.com");
    println!("User {} email {}", 
        user1.username,
        user1.email);
    let user2 = build_user(
        String::from("steffisiegert@test.com"),
        String::from("ssi")
        );
    println!("User {} email {} ",
            user2.username,
            user2.email);

    let user4 = User {
        email: String::from("yetanother@email.com"),
        username: String::from("kleineMaus"),
        ..user2
    };
    println!("User {} email {} ",
            user4.username,
            user4.email);
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin= Point(0, 0, 0);
}
