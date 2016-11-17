#[macro_use]

extern crate clap;
extern crate chrono;
extern crate tick;
extern crate diesel;

use self::tick::*;
use self::tick::models::*;
use self::diesel::prelude::*;

use clap::App;
//use chrono::*;

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
        ( "start", Some( options ) ) => {
            //use tick::schema::timers::dsl::*;
            let timer_name = options.value_of( "name" );
            let timer_entry = options.value_of( "entry" );
            if verbosity {
                println!( "Starting a timer for `{}` with message \"{}\".", timer_name.unwrap(), timer_entry.unwrap() );
            }
        },
        ( "stop", Some( options ) ) => {
            //use tick::schema::timers::dsl::*;
            let timer_name = options.value_of( "name" );
            let timer_entry = options.value_of( "entry" );
            if verbosity {
                println!( "Ending a timer for `{}` with message \"{}\".", timer_name.unwrap(), timer_entry.unwrap() );
            }
        },
        ( "data", Some( options ) ) => {
            use tick::schema::timers::dsl::*;
            let connection = establish_connection();
            let results = timers.order( id.asc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers" );
            println!( "Displaying {} timers", results.len() );
            for timer in results {
                println!( "{:?}", timer )
            }
        },
        ( "status", Some( options ) ) => {
            use tick::schema::timers::dsl::*;
            let connection = establish_connection();
            let results = timers.filter( end_time.is_null() )
                .order( id.desc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers" );
            match options.occurrences_of( "all" ) {
                1 => {
                    println!( "Status for all timers:" );
                    for timer in results {
                        println!( "{:?}", timer )
                    }
                },
                _ => println!( "Status for latest timer: {:?}", results.first() ),
            }
        },
        _ => (),
    };
}
