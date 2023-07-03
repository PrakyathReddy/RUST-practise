#[allow(unused)]
struct MyStruct<A, B> {
    a: A,
    b: B,
}
#[allow(unused)]
enum MyEnum<A, B> {
    A(A),
    B(B),
}
#[allow(unused)]
fn main() {
    let s = MyStruct { a: 10, b: "hello" };
    let e = MyEnum::<i32, _>::B("hello");
}
