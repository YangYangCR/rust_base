fn main() {
    let mut user1 = User {
        email: String::from(""),
        username: String::from("Tom"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("anotheremail@example.com");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}