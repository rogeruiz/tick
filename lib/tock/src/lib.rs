#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var( "DATABASE_URL" )
        .expect( "DATABASE_URL expected to be set in the environment" );
    SqliteConnection::establish( &database_url )
        .expect( &format!( "Error connecting to {}", database_url ) )
}
