fn main() {
    use std::collections::HashSet;

    let mut cool_numbers = HashSet::from([2, 10, 8]);
    // inserting a number that already exists will have no effect
    cool_numbers.insert(8);

    println!("{:?}", cool_numbers);
    println!("{:?}", &cool_numbers - &HashSet::from([2]));
}
