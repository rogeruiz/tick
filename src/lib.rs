#![feature(proc_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{ Timer, NewTimer };
use chrono::*;

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_file = env::var( "DATABASE_URL" )
        .expect( "DATABASE_URL must be set in the environment" );
    SqliteConnection::establish( &database_file )
        .expect( &format!( "Error connection to {}", database_file ))
}

pub fn create_timer<'a>( conn: &SqliteConnection, name: &'a str, start_entry: &'a str ) {
    use schema::timers;

    let start = Local::now().format( "%Y-%m-%d %H:%M:%S" ).to_string();

    println!( "Timer Starting at {} ", start );

    let new_timer = NewTimer {
        name: name,
        start_time: &start,
        start_entry: start_entry,
    };

    diesel::insert( &new_timer ).into( timers::table )
        .execute( conn )
        .expect( "Error saving new timer." );
}
