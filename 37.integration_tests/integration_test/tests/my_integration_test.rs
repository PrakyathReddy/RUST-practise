#![allow(unused)]
fn main() {
    #[test]
    fn test_add() {
        assert_eq!(integration_test::add(3,2), 5);
    }
}