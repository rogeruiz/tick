#![feature(proc_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_file = env::var( "DATABASE_URL" )
        .expect( "DATABASE_URL must be set in the environment" );
    SqliteConnection::establish( &database_file )
        .expect( &format!( "Error connection to {}", database_file ))
}

