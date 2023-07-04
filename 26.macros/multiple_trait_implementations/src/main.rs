fn main() {
    #[derive(Debug, Clone, Default, Eq, PartialEq)]
    struct MyType {
        name: String,
        items: Vec<i32>,
    }

    let v1 = MyType::default();
    let v2 = v1.clone();
    assert_eq!(&v1, &v2);
    println!("{v1:#?}");
}
