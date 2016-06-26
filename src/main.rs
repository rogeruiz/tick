#[macro_use]

extern crate clap;
extern crate chrono;

use clap::App;
use chrono::*;

mod relay;

fn main() {

    let tick = load_yaml!( "config/tick.yml" );

    let matches = App::from_yaml( tick )
        .about( env!( "CARGO_PKG_DESCRIPTION" ) )
        .version( crate_version!() )
        .get_matches();

    let be_verbose = matches.is_present( "verbose" );

    let mut name = matches.value_of( "name" )
        .unwrap_or( "" );

    let mut entry = matches.value_of( "entry" )
        .unwrap_or( "" );

    if be_verbose && name != "" {
        println!( "Using default name '{}' for timers", name );
    }

    //let mut timers_db = database::Database::create( be_verbose );

    match matches.subcommand() {
        ( "start", Some( sub_m ) ) => {
            let mut query = relay::Query {
                name: name,
                start_time: &Local::now(),
                end_time: &Local::now(),
                start_entry: entry,
                end_entry: "",
                verbose: &be_verbose,
            };
            if sub_m.is_present( "name" ) as bool {
                query.name = sub_m.value_of( "name" ).unwrap();
                if be_verbose {
                    println!( "Updating name '{}' for timers", query.name );
                }
            }
            if sub_m.is_present( "entry" ) as bool {
                query.start_entry = sub_m.value_of( "entry" ).unwrap();
                if be_verbose {
                    println!( "Updating entry '{}' for timers", query.start_entry );
                }
            }
            println!(
                "Starting timer for '{}' @ '{}' with message {}",
                query.name,
                query.start_time,
                query.start_entry
            );

            query.start_timer();
        },
        //Some( "stop" ) => {
            //let current_time = Local::now();
            //if matches.is_present( "name" ) as bool {
                //name = matches.value_of( "name" ).unwrap();
                //if be_verbose {
                    //println!( "Updating name '{}' for timers", name );
                //}
            //}
            //if be_verbose {
                //println!(
                    //"Stopping timer for {} @ {}",
                    //name,
                    //current_time
                //);
            //}
        //},
        //Some( "data" ) => {
            //if matches.is_present( "type" ) as bool {
                //if be_verbose {
                    //println!(
                        //"Data type selected for output {}",
                        //matches.value_of( "type" ).unwrap()
                    //);
                //}
            //}
            //if matches.is_present( "name" ) as bool {
                //name = matches.value_of( "name" ).unwrap();
                //if be_verbose {
                    //println!( "Using new name '{}' for timers", name );
                //}
            //}
            //if be_verbose {
                //println!(
                    //"Searching for {}",
                    //name
                //);
            //}
        //},
        _ => (),
    };

}
