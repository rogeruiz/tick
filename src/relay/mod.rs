extern crate sqlite;

use std::env;
use std::fs;
use std::path::PathBuf;
use self::sqlite::Connection;
//use self::sqlite::State;
use chrono::*;

pub struct Query<'q> {
    pub name: &'q str,
    pub start_time: &'q DateTime<Local>,
    pub end_time: &'q DateTime<Local>,
    pub start_entry: &'q str,
    pub end_entry: &'q str,
    pub verbose: &'q bool,
}

/*
 * Create Database
 */
pub fn create( be_verbose: bool ) -> Connection {
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
            Err( _ ) => {
                if be_verbose {
                    panic!( "An error occurred creating database directory!" )
                }
            },
        }
    }

    let connection = sqlite::open( database_file ).unwrap();
    let query = "
       CREATE TABLE timers (
            id INTEGER,
            name TEXT,
            start_time TEXT,
            end_time TEXT,
            start_entry TEXT,
            end_entry TEXT
        );
    ";

    match connection.execute( query ) {
        Ok( _ ) => (),
        Err( e ) => {
            if be_verbose {
                println!( "Table already exists {:?}", e );
            }
        },
    }

    connection

}


impl<'q> Query<'q> {

    fn start( &self ) -> String {

        format!(
            "
                INSERT INTO timers (
                    name,
                    start_time,
                    end_time,
                    start_entry
                ) VALUES (
                    '{}',
                    '{}',
                    'NOW',
                    '{}'
                );
            ",
            self.name,
            self.start_time,
            self.start_entry
        )

    }

    /*
     * Write to the database
     */
    pub fn start_timer( &self ) -> Connection {

        let start = Query::start( &self );
        let db = create( true );


        match db.execute( start ) {
            Ok( _ ) => (),
            Err( e ) => println!( "Something went wrong, {}", e ),
        }

        db

    }

}
