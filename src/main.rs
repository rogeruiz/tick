#[ macro_use ] extern crate clap;

use std::env;
use std::process;
use clap::App;

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
    let connection = establish_connection();

    match matches.subcommand() {
        ( "start", Some( o ) ) => {
            let n = o.value_of( "name" ).unwrap_or( "" );
            let e = o.value_of( "entry" ).unwrap_or( "" );

            if n == "" {
                println!( "Cannot start a timer without a name: {}", &n );
                process::exit(99);
            }

            if verbosity {
                println!( "Starting a timer for `{}` with message \"{}\".", &n, &e );
            }
            create_timer( &connection, &n, &e );
            if verbosity {
                println!( "Started timer for {}", n );
            }
        },
        ( "stop", Some( o ) ) => {
            use schema::timers::dsl::*;
            let n = o.value_of( "name" ).unwrap_or( "" );
            let e = o.value_of( "entry" ).unwrap_or( "" );
            if verbosity {
                if n == "" {
                    println!( "Ending latest running timer with message \"{}\"", &e );
                } else {
                    println!( "Ending a timer for `{}` with message \"{}\".", &n, &e );
                }
            }

            stop_timer( &n, &e );
        },
        ( "list", _ ) => {
            use schema::timers::dsl::*;
            let results = timers.order( id.asc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers table" );
            if verbosity {
                println!( "Displaying {} timers", results.len() );
            }
            for timer in results {
                println!(
                    "{timer_id} {start_date} [ {start_time} - {end_time} ] ( {duration} ) [ {timer_name} ]",
                    timer_id=timer.id,
                    start_date=parse_date( timer.start_time ),
                    start_time=parse_time( timer.start_time ),
                    end_time=parse_time( timer.end_time ),
                    duration=get_duration( timer.start_time, timer.end_time ),
                    timer_name=timer.name
                );
                if verbosity {
                    println!( "message(s):\n{}\n{}", timer.start_entry, timer.end_entry );
                }
            }
        },
        ( "status", _ ) => {
            use schema::timers::dsl::*;
            let results = timers.filter( running.eq( 1 ) )
                .order( id.desc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers" );

            if results.len() > 0 {
                let timer = results.first().unwrap();
                if verbosity {
                    println!(
                        "{timer_id} {start_date} [ {start_time} - {end_time} ] ( {duration} ) [ {timer_name} ]",
                        timer_id=timer.id,
                        start_date=parse_date( timer.start_time ),
                        start_time=parse_time( timer.start_time ),
                        end_time=parse_time( timer.end_time ),
                        duration=get_duration( timer.start_time, timer.end_time ),
                        timer_name=timer.name
                    );
                    println!( "message(s):\n{} {}", timer.start_entry, timer.end_entry );
                } else {
                    println!( "elapsed time: {}", get_duration( timer.start_time, timer.end_time ) );
                }
            } else {
                println!( "No timers currently running" );
                process::exit(99);
            }
        },
        ( "remove", Some( o ) ) => {
            use schema::timers::dsl::*;
            let i: i32 = value_t!( o, "id", i32 ).unwrap_or( 0 );

            if i < 1 {
                println!( "Cannot remove timers without a proper id." );
                process::exit(99);
            }

            if verbosity {
                println!( "Removing timer with matching id {}", &i );
            }

            let _ = diesel::delete( timers.find( &i ) )
                .execute( &connection )
                .expect( &format!( "Unable to remove timer matching id {}", &i ) );
        },
        _ => (),
    };
}
