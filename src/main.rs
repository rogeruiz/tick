#[ macro_use ] extern crate diesel_codegen;
pub mod schema;
pub mod models;
#[ macro_use ] extern crate diesel;
#[ macro_use ] extern crate clap;
extern crate dotenv;
extern crate chrono;

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::process;

use clap::App;

use models::*;
use schema::timers;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var( "DATABASE_URL" )
        .expect( "DATABASE_URL expected to be set in the environment" );
    SqliteConnection::establish( &database_url )
        .expect( &format!( "Error connecting to {}", database_url ) )
}

fn create_timer<'a>( conn: &SqliteConnection, name: &'a str, start_entry: &'a str ) -> usize {

    let new_timer = NewTimer {
        name: name,
        start_time: Local::now().timestamp() as i32,
        start_entry: start_entry,
        running: 1,
    };

    diesel::insert( &new_timer )
        .into( timers::table )
        .execute( conn )
        .expect( "Error saving new timer" )
}

fn parse_date<'a>( ts: i32 ) -> String {
    let timestring = format!( "{:?}", ts );
    let dt: DateTime<Local> = Local.datetime_from_str( &timestring, "%s" ).unwrap();
    dt.format( "%Y-%m-%d" ).to_string()
}

fn parse_time<'a>( ts: i32 ) -> String {
    let timestring = format!( "{:?}", ts );
    let dt: DateTime<Local> = Local.datetime_from_str( &timestring, "%s" ).unwrap();
    if ts == 0 {
        format!( "NOW" )
    } else {
        dt.format( "%H:%M:%S" ).to_string()
    }
}

fn get_duration<'a>( s: i32, e: i32 ) -> String {
    let mut now: i32 = Local::now().timestamp() as i32;
    if e > s {
        now = e;
    }
    let delta = now - s;
    format!(
        "{hours:02}:{minutes:02}:{seconds:02}",
        hours=delta / 60 / 60,
        minutes=delta / 60 % 60,
        seconds=delta % 60
    )
}

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

            let timer: std::result::Result<models::Timer, diesel::result::Error> = timers.filter( name.like( &n ) )
                .filter( running.eq( 1 ) )
                .first( &connection );

            match timer {
                Ok( t ) => {
                    let _ = diesel::update( timers.find( &t.id ) )
                        .set( ( running.eq( 0 ), end_time.eq( Local::now().timestamp() as i32 ), end_entry.eq( &e ) ) )
                        .execute( &connection )
                        .expect( &format!( "Unable to stop timer {}", &t.id ) );
                    if verbosity {
                        println!( "Stopped timer for {}", &t.name );
                    }
                },
                Err( err ) => {
                    if verbosity {
                        println!( "{} running timers matching {}, so attempting to stop lastest running timer.", &err, &n );
                    }
                    let latest_timer: std::result::Result<models::Timer, diesel::result::Error> = timers.filter( running.eq( 1 ) ).first( &connection );
                    match latest_timer {
                        Ok( lt ) => {
                            let _ = diesel::update( timers.find( &lt.id ) )
                                .set( ( running.eq( 0 ), end_time.eq( Local::now().timestamp() as i32 ), end_entry.eq( &e ) ) )
                                .execute( &connection )
                                .expect( "Unable to stop latest running timer." );
                            if verbosity {
                                println!( "Stopped latest running timer for {}", &lt.name );
                            }
                        },
                        _ => (),
                    }
                }
            }
        },
        ( "list", Some( o ) ) => {
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
        ( "status", Some( o ) ) => {
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
