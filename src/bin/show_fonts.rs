extern crate font_be;
extern crate diesel;

use self::font_be::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use font_be::schema::fonts::dsl::*;
    let connection = establish_connection();

    let results = fonts.load::<Fonts>(&connection)
        .expect("Error loading Fonts");

    println!("Displaying {} Fonts", results.len());
    for item in results {
        println!("{}", item.id);
        println!("----------\n");
        println!("{}", item.name);
    }
}