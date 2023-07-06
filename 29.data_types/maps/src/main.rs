fn main() {
    struct UserProfile {
        name: String,
        age: i32,
    }

    let users = vec![
        UserProfile {
            name: "Mick".to_string(),
            age: 30,
        },
        UserProfile {
            name: "Jenny".to_string(),
            age: 22,
        },
    ];

    println!(
        "{:?}",
        users
            .iter()
            .find(|profile| profile.name == "Mick")
            .unwrap()
            .age
    );
}
