#![allow(unused)]
fn main() {
    #[derive(Default)]
    pub struct Database {
        count: u32,
    }

    impl Database {
        pub fn operate(&mut self) {
            self.count += 1;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        impl Database {
            fn at_10() -> Database {
                Self { count: 10 }
            }
        }

        #[test]
        fn operate_once() {
            let mut database = Database::at_10();
            database.operate();
            assert_eq!(database.count, 11);
        }

        #[test]
        fn operating_twice() {
            let mut database = Database::at_10();
            database.operate();
            database.operate();
            assert_eq!(database.count, 12);
        }
    }
}