use rand;

mod plant;
mod sound;
mod menu;

use crate::menu::Appetizer;

fn main() {
    // Structs can be pub, but fields must be made pub too
    let carrot = plant::Vegetable::new("carrot");
    println!("My Vegetable: {}", carrot.name);

    // Pub enums allow access to values
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    let my_order = if rand::random() { order1 } else { order2 };

    match my_order {
        Appetizer::Soup => menu::print_order("Soup"),
        Appetizer::Salad => menu::print_order("Salad"),
    }

    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
