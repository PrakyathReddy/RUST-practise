fn twice<F: Fn() -> &'static str>(f: F) {
    println!("{} {}", f(), f());
}

fn main() {
    twice(|| "One");
}

// --- FnMut
// fn twice<F: FnMut() -> &'static str>(mut f: F) {
//     println!("{} {}", f(), f());
// }

// fn main() {
//     let mut iter = ["One", "Two"].into_iter();
//     twice(|| iter.next().unwrap()); // One Two
// }


// --- FnOnce
// fn once<F: FnOnce() -> String>(f: F) {
//     println!("{}", f());
// }

// fn main() {
//     let one = String::from("One");
//     once(move || one); // One
// }