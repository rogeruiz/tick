#[macro_use]

extern crate clap;
extern crate chrono;
extern crate tick;
extern crate diesel;

use self::tick::*;
use self::tick::models::*;
use self::diesel::prelude::*;

use clap::App;
use chrono::*;

/*
 * The main function which sets up the CLI and calls the match handlers
 */
fn main () {
    let config = load_yaml!( "config.yml" );
    let matches = App::from_yaml( config )
        .about( env!( "CARGO_PKG_DESCRIPTION" ) )
        .version( crate_version!() )
        .get_matches();
    let verbosity = matches.is_present( "verbose" );

    match matches.subcommand() {
        ( "data", Some( options ) ) => {

            use tick::schema::timers::dsl::*;

            if verbosity {
                println!( "Starting a timer, with options ({:?})", options );
            }
            let connection = establish_connection();
            let results = timers.load::<Timer>( &connection )
                .expect( "Error loading timers" );

            println!( "Displaying {} timers", results.len() );
            for timer in results {
                println!( "Id: {:?}", timer.id);
                println!( "-------------------\n");
                println!( "Name: {:?}", timer.name);
                println!( "-------------------\n");
                println!( "Start Time: {:?}", timer.start_time);
                println!( "-------------------\n");
                println!( "End Time: {:?}", timer.end_time);
                println!( "-------------------\n");
                println!( "Start Entry: {:?}", timer.start_entry);
                println!( "-------------------\n");
                println!( "End Entry: {:?}", timer.end_entry);
            }

        },
        _ => (),
    };
}
