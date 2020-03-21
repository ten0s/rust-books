mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

// absolute use
//use crate::front_of_house::hosting;

// relative use
//use front_of_house::hosting;

// pub absolute use?
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();
    // relative
    front_of_house::hosting::add_to_waitlist();
    // use(s)
    hosting::add_to_waitlist();

    // order
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind
    meal.toast = String::from("Wheat");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
