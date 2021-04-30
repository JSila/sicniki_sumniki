#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

pub mod handlers;
pub mod service;
mod util;

pub fn establish_connection() -> MysqlConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
