#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod schema;
pub mod models;

extern crate dotenv;
extern crate chrono;

use std::env;
use dotenv::dotenv;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use models::*;
use schema::timers;

embed_migrations!("../../migrations");

// establish_connection returns a SqliteConnection to the
// TICK_DATABASE_FILE environment variable
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_path = env::var("TICK_DATABASE_FILE")
        .expect("TICK_DATABASE_FILE expected to be set in the environment");

    let connection = SqliteConnection::establish(&database_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_path));

    // Run the migrations for the database
    let _ = embedded_migrations::run(&connection);

    // Return the SqliteConnection
    connection
}

// create_timer takes a conenction and a name and start_entry string and creates
// a new timer.
pub fn create_timer<'a>(conn: &SqliteConnection, name: &'a str, start_entry: &'a str) -> usize {
    let new_timer = NewTimer {
        name,
        start_time: Local::now().timestamp() as i32,
        start_entry,
        running: 1,
    };

    diesel::insert_into(timers::table)
        .values(&new_timer)
        .execute(conn)
        .expect("Error saving new timer")
}

// latest_timer is a private function which gets the latest running timer by
// timer_name or the latest timer by "running" being true.
fn latest_timer<'a>(conn: &'a SqliteConnection, timer_name: &'a str) -> Result<models::Timer, diesel::result::Error> {
    use schema::timers::dsl::*;

    if timer_name != "" {
        timers.filter(name.like(&timer_name))
            .filter(running.eq(1))
            .first(conn)
    } else {
        timers.filter(running.eq(1))
            .first(conn)
    }
}

// stop_timer takes a connection and a name and end_entry string and stops a
// running timer.
pub fn stop_timer<'a>(conn: &'a SqliteConnection, timer_name: &'a str, timer_end_entry: &'a str) -> () {
    use schema::timers::dsl::*;

    let timer = latest_timer(&conn, &timer_name);

    match timer {
        Ok(t) => {
            diesel::update(timers.find(&t.id))
                .set((
                    running.eq(0),
                    end_time.eq(Local::now().timestamp() as i32),
                    end_entry.eq(&timer_end_entry)
                ))
                .execute(conn)
                .expect(&format!("Unable to stop timer {}", &t.id));
        },
        Err(_) => println!("Are you sure a timer is running? Better go catch it.")
    }

}

// list_timers takes a connection and returns ascending order timers.
pub fn list_timers<'a>(conn: &'a SqliteConnection) -> Vec<models::Timer> {
    use schema::timers::dsl::*;

    timers.order(id.asc())
        .load::<Timer>(conn)
        .expect("Error loading timers table")
}

// check_timer takes a connection and returns descending order running timers.
pub fn check_timer<'a>(conn: &'a SqliteConnection) -> Vec<models::Timer> {
    use schema::timers::dsl::*;

    timers.filter(running.eq(1))
        .order(id.desc())
        .load::<Timer>(conn)
        .expect("Error getting running timer")
}

// remove_timer takes a connection and an ID and deletes the timer matching the ID.
pub fn remove_timer<'a>(conn: &'a SqliteConnection, lookup_id: &'a i32) -> usize {
    use schema::timers::dsl::*;

    diesel::delete(timers.find(&lookup_id))
        .execute(conn)
        .expect(&format!("Unable to remove timer matching id {}", &lookup_id))
}

// parse_date takes a timestamp number and returns a date-formatted string.
pub fn parse_date<'a>(ts: i32) -> String {
    let timestring = format!("{:?}", ts);
    let dt: DateTime<Local> = Local.datetime_from_str(&timestring, "%s").unwrap();
    dt.format("%Y-%m-%d").to_string()
}

// parse_time takes a timestamp number and returns a time-formatted string.
pub fn parse_time<'a>(ts: i32) -> String {
    let timestring = format!("{:?}", ts);
    let dt: DateTime<Local> = Local.datetime_from_str(&timestring, "%s").unwrap();
    if ts == 0 {
        format!("NOW")
    } else {
        dt.format("%H:%M:%S").to_string()
    }
}

// get_duration takes a start and end timestamp number and returns the delta
// between the start and end timestamp as a time-formatted string.
pub fn get_duration<'a>(s: i32, e: i32) -> String {
    let mut now: i32 = Local::now().timestamp() as i32;
    if e > s {
        now = e;
    }
    let delta = now - s;
    format!(
        "{hours:02}:{minutes:02}:{seconds:02}",
        hours = delta / 60 / 60,
        minutes = delta / 60 % 60,
        seconds = delta % 60
    )
}

