diesel::table! {
    timers (id) {
        id -> Integer,
        name -> VarChar,
        start_time -> Integer,
        end_time -> Integer,
        start_entry -> Text,
        end_entry -> Text,
        running -> Integer,
    }
}
