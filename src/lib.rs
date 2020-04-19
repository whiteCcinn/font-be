pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Fonts, NewFonts};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, name: &str, created_at: i32, updated_at: i32) -> Fonts {
    use schema::fonts;

    let new_fonts = NewFonts {
        name,
        created_at,
        updated_at,
    };

    diesel::insert_into(fonts::table)
        .values(&new_fonts)
        .get_result(conn)
        .expect("Error saving new fonts")
}