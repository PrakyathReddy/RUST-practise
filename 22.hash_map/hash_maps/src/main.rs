fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);
    scores.insert(String::from("yellow"), 50); 
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    let team_name = String::from("yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    let text = String::from("Hello world, wonderful world");
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}