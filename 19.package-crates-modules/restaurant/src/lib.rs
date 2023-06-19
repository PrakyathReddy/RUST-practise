mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // after creating 'use'
    hosting::add_to_waitlist(); 

    let order1 = back_of_house::Appetizer::Soup;
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String;
        seasonal_fruits: String::from("Peaches"),
    }

    impl Breakfast {
        pub fn eat_at_restaurant() {
            let mut meal = back_of_house::Breakfast::summer("Rye");
            meal.toast = String::from("Wheat");
            println!("I'd like {} toast please", meal.host);
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad,
    }
}