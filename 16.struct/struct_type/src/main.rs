fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_account: u64,
    }


    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someemail@gmail.com"),
        sign_in_account: 1,
    };

    user1.email = String::from("anothermail@gmail");

    let user2 = User {
        email: String::from("user2@gmail"),
        ..user1
    };
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_account: 1,
//     }
// }

