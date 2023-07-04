#![allow(unused)]
fn main() {
    #[derive(Default)]
    struct MyType {
        name: String,
        items: Vec<i32>,
    }

    let v = MyType::default();
    assert!(v.name.is_empty());
    assert!(v.items.is_empty());
}

/*
 *
 * the generated implementation for MyType will look like this:
 * impl Default for MyType {
 *   fn default() -> Self {
 *       Self {
 *           name: Default::default(),
 *           items: Default::default(),
 *       }
 *   }
 * }
 *
 */