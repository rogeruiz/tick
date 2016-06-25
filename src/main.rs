#[macro_use]

extern crate clap;
extern crate chrono;

use clap::App;
use chrono::*;

mod database;
//mod memo;
//pub mod timer;
//mod relay;

fn main() {

    let tick = load_yaml!( "config/tick.yml" );

    let matches = App::from_yaml( tick )
        .about( env!( "CARGO_PKG_DESCRIPTION" ) )
        .version( crate_version!() )
        .get_matches();

    let be_verbose = matches.is_present( "verbose" );

    let mut name = matches.value_of( "name" )
        .unwrap_or( "" );

    if be_verbose && name != "" {
        println!( "Using default name '{}' for timers", name );
    }

    //let mut timers_db = database::Database::create( be_verbose );
    let db = Database::create( be_verbose );

    if let Some( matches ) = matches.subcommand_matches( "start" ) {
        let current_time = Local::now();
        if matches.is_present( "name" ) as bool {
            name = matches.value_of( "name" ).unwrap();
            if be_verbose {
                println!( "Updating name '{}' for timers", name );
            }
        }
        println!(
            "Starting timer for '{}' @ '{}'",
            name,
            current_time
        );
        //timers_db = database::write_start( timers_db, be_verbose );
        //timers_db.( query ).unwrap();
    }

    if let Some( matches ) = matches.subcommand_matches( "stop" ) {
        let current_time = Local::now();
        if matches.is_present( "name" ) as bool {
            name = matches.value_of( "name" ).unwrap();
            if be_verbose {
                println!( "Updating name '{}' for timers", name );
            }
        }
        if be_verbose {
            println!(
                "Stopping timer for {} @ {}",
                name,
                current_time
            );
        }
    }

    if let Some( matches ) = matches.subcommand_matches( "data" ) {
        if matches.is_present( "type" ) as bool {
            if be_verbose {
                println!(
                    "Data type selected for output {}",
                    matches.value_of( "type" ).unwrap()
                );
            }
        }
        if matches.is_present( "name" ) as bool {
            name = matches.value_of( "name" ).unwrap();
            if be_verbose {
                println!( "Using new name '{}' for timers", name );
            }
        }
        if be_verbose {
            println!(
                "Searching for {}",
                name
            );
        }
    }

}
