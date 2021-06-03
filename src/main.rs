#[macro_use]
extern crate clap;
extern crate tock;

use std::env;
use std::process;
use clap::App;

/*
 * The main function which sets up the CLI and calls the match handlers
 */
fn main() {
    let config = load_yaml!("config.yml");
    let matches = App::from_yaml(config)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(crate_version!())
        .get_matches();
    let verbosity = matches.is_present("verbose");
    let connection = tock::establish_connection();

    match matches.subcommand() {
        ("start", Some(o)) => {
            let n = o.value_of("name").unwrap_or("");
            let e = o.value_of("entry").unwrap_or("");

            if n == "" {
                println!("Cannot start a timer without a name");
                process::exit(99);
            }

            if verbosity {
                println!("Starting a timer for `{}` with message \"{}\".", &n, &e);
            }
            tock::create_timer(&connection, &n, &e);
            if verbosity {
                println!("Started timer for {}", n);
            }
        }
        ("stop", Some(o)) => {
            let n = o.value_of("name").unwrap_or("");
            let e = o.value_of("entry").unwrap_or("");
            if verbosity {
                if n == "" {
                    println!("Ending latest running timer with message \"{}\"", &e);
                } else {
                    println!("Ending a timer for `{}` with message \"{}\".", &n, &e);
                }
            }

            tock::stop_timer(&connection, &n, &e);
        }
        ("list", _) => {
            let results = tock::list_timers(&connection);

            if verbosity {
                println!("Displaying {} timers", results.len());
            }
            for timer in results {
                println!(
                    "{timer_id} {start_date} [ {start_time} - {end_time} ] ( {duration} ) [ {timer_name} ]",
                    timer_id=timer.id,
                    start_date=tock::parse_date( timer.start_time ),
                    start_time=tock::parse_time( timer.start_time ),
                    end_time=tock::parse_time( timer.end_time ),
                    duration=tock::get_duration( timer.start_time, timer.end_time ),
                    timer_name=timer.name
                );
                if verbosity {
                    println!("message(s):\n{}\n{}", timer.start_entry, timer.end_entry);
                }
            }
        }
        ("status", _) => {
            let results = tock::check_timer(&connection);

            if results.len() > 0 {
                let timer = results.first().unwrap();
                if verbosity {
                    println!(
                        "{timer_id} {start_date} [ {start_time} - {end_time} ] ( {duration} ) [ {timer_name} ]",
                        timer_id=timer.id,
                        start_date=tock::parse_date( timer.start_time ),
                        start_time=tock::parse_time( timer.start_time ),
                        end_time=tock::parse_time( timer.end_time ),
                        duration=tock::get_duration( timer.start_time, timer.end_time ),
                        timer_name=timer.name
                    );
                    println!("message(s):\n{} {}", timer.start_entry, timer.end_entry);
                } else {
                    println!(
                        "elapsed time: {}",
                        tock::get_duration(timer.start_time, timer.end_time)
                    );
                }
            } else {
                println!("No timers currently running");
                process::exit(99);
            }
        }
        ("remove", Some(o)) => {
            let i: i32 = value_t!(o, "id", i32).unwrap_or(0);

            if i < 1 {
                println!("Cannot remove timers without a proper id.");
                process::exit(99);
            }

            if verbosity {
                println!("Removing timer with matching id {}", &i);
            }

            tock::remove_timer(&connection, &i);
        }
        _ => (),
    };
}
