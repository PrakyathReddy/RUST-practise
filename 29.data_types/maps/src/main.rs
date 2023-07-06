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

    let name_to_profile: std::collections::HashMap<String, UserProfile> = users
        .into_iter()
        .map(|profile| (profile.name.clone(), profile))
        .collect();

    println!("{:?}", name_to_profile["Mick"].age);
}
