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

    match matches.subcommand() {
        ( "start", Some( o ) ) => {
            let n = o.value_of( "name" ).unwrap();
            let e = o.value_of( "entry" ).unwrap_or( "" );
            if verbosity {
                println!( "Starting a timer for `{}` with message \"{}\".", &n, &e );
            }
            let connection = establish_connection();
            create_timer( &connection, &n, &e );
            println!( "Started timer for {}", n );
        },
        ( "stop", Some( o ) ) => {
            use schema::timers::dsl::*;
            let n = o.value_of( "name" ).unwrap_or( "" );
            let e = o.value_of( "entry" ).unwrap_or( "" );
            if verbosity {
                if n == "" {
                    println!( "Ending latest running timer" );
                } else {
                    println!( "Ending a timer for `{}` with message \"{}\".", &n, &e );
                }
            }

            let connection = establish_connection();

            let timer: std::result::Result<models::Timer, diesel::result::Error> = timers.filter( name.like( &n ) )
                .filter( running.eq( 1 ) )
                .first( &connection );

            match timer {
                Ok( t ) => {
                    let _ = diesel::update( timers.find( &t.id ) )
                        .set( ( running.eq( 0 ), end_time.eq( Local::now().timestamp() as i32 ), end_entry.eq( &e ) ) )
                        .execute( &connection )
                        .expect( &format!( "Unable to stop timer {}", &t.id ) );
                },
                Err( err ) => {
                    if verbosity {
                        println!( "{} running timers matching {}", &err, &n );
                    }
                    let latest_timer: std::result::Result<models::Timer, diesel::result::Error> = timers.filter( running.eq( 1 ) ).first( &connection );
                    match latest_timer {
                        Ok( lt ) => {
                            let _ = diesel::update( timers.find( &lt.id ) )
                                .set( ( running.eq( 0 ), end_time.eq( Local::now().timestamp() as i32 ), end_entry.eq( &e ) ) )
                                .execute( &connection )
                                .expect( "Unable to stop latest running timer" );
                        },
                        _ => (),
                    }
                }
            }
        },
        ( "list", Some( o ) ) => {
            use schema::timers::dsl::*;
            let connection = establish_connection();
            let results = timers.order( id.asc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers table" );
            println!( "Displaying {} timers", results.len() );
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
                    println!( "message(s):\n{} {}", timer.start_entry, timer.end_entry );
                }
            }
        },
        ( "status", Some( o ) ) => {
            use schema::timers::dsl::*;
            let connection = establish_connection();
            let results = timers.filter( running.eq( 1 ) )
                .order( id.desc() )
                .load::<Timer>( &connection )
                .expect( "Error loading timers" );

            if results.len() > 0 {
                let timer = results.first().unwrap();
                println!(
                    "{timer_id} {start_date} [ {start_time} - {end_time} ] ( {duration} ) [ {timer_name} ]",
                    timer_id=timer.id,
                    start_date=parse_date( timer.start_time ),
                    start_time=parse_time( timer.start_time ),
                    end_time=parse_time( timer.end_time ),
                    duration=get_duration( timer.start_time, timer.end_time ),
                    timer_name=timer.name
                );
            }
        },
        _ => (),
    };
}
