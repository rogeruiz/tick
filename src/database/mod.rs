extern crate sqlite;

use std::env;
use std::fs;
use std::path::PathBuf;
use database::sqlite::Connection;

pub fn create_and_open( be_verbose: bool ) -> Connection {
    let mut data_dir = PathBuf::new();
    data_dir.push( env::home_dir().unwrap() );
    data_dir.push( ".tick/db" );
    let database_file = format!( "{}/test.db", data_dir.display() );
    if ! data_dir.exists() {
        if be_verbose {
            println!( "Creating a database for tick at location: {}", data_dir.display() );
        }
        match fs::create_dir_all( data_dir ) {
            Ok( _ ) => (),
            Err( _ ) => panic!( "An error occurred creating database directory!" ),
        }
    }
    let connection = sqlite::open( database_file ).unwrap();

    // TODO: Check if timers TABLE already exists because this fails after the first time
    let init_table = "
        CREATE TABLE timers (
            id INTEGER,
            name TEXT,
            start_time TEXT,
            end_time TEXT,
            start_entry TEXT,
            end_entry TEXT
        );
    ";
    match connection.execute( init_table ) {
        Ok( _ ) => (),
        Err( e ) => println!( "error! {:?}", e ),
    }

    connection
}
