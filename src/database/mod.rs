extern crate sqlite;

use std::env;
use std::fs;
use std::path::PathBuf;
use database::sqlite::Connection;
use database::sqlite::State;

pub struct Database {
    pub query: String,
    pub connection: Connection,
    pub result: State,
}

impl Database {

    fn table_query() -> String {
        format!( "
           CREATE TABLE timers (
                id INTEGER,
                name TEXT,
                start_time TEXT,
                end_time TEXT,
                start_entry TEXT,
                end_entry TEXT
            );
        " )
    }

    pub fn start_query( name: &'static str, time: &'static str ) -> String {

        format!( "
            INSERT INTO timers (
                name,
                start_time,
                end_time
            ) VALUES (
                '{}',
                '{}',
                'NOW'
            );
        ", name, time )

    }

    /*
     * Create Database
     */
    pub fn create( &self, be_verbose: bool ) -> &Database {
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

        self.connection = sqlite::open( database_file ).unwrap();
        self.query = Database::table_query();

        match self.connection.execute( &self.query ) {
            Ok( _ ) => (),
            Err( e ) => {
                if be_verbose {
                    println!( "Table already exists {:?}", e );
                }
            },
        }

        self

    }

    /*
     * Write to the database
     */
    //pub fn write( db: Connection, be_verbose: bool ) -> Connection {

        //db

    //}

}
