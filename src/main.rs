#[macro_use]

extern crate clap;
//extern crate sqlite;

use clap::App;

fn main() {

    //let connection = sqlite::open( ":memory:" )
        //.unwrap();

    let tick = load_yaml!( "config/tick.yml" );

    let matches = App::from_yaml( tick )
        .about( env!( "CARGO_PKG_DESCRIPTION" ) )
        .version( crate_version!() )
        .get_matches();

    let name = matches.value_of( "name" )
        .unwrap_or( "" );

    println!( "Using the name: {}", name );

    match matches.occurrences_of( "v" ) {
        0 => println!( "No verbose info" ),
        1 => println!( "Some verbose info" ),
        2 => println!( "Tons of verbose info" ),
        3 | _ => println!( "Don't be crazy" ),
    }

    if let Some( matches ) = matches.subcommand_matches( "start" ) {
        let new_name = matches.value_of( "name" ).unwrap_or( "" );
        if matches.is_present( "name" ) as bool {
            println!( "Overriding {} with {}", name , new_name );
        }
    }

    if let Some( matches ) = matches.subcommand_matches( "stop" ) {
        let new_name = matches.value_of( "name" ).unwrap_or( "" );
        if matches.is_present( "name" ) as bool {
            println!( "Overriding {} with {}", name , new_name );
        }
    }

}
