#[ derive( Queryable ) ]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub start_time: i32,
    pub end_time: i32,
    pub start_entry: String,
    pub end_entry: String,
    pub running: i32,
}

use super::schema::timers;

#[ derive( Insertable ) ]
#[ table_name="timers" ]
pub struct NewTimer<'a> {
    pub name: &'a str,
    pub start_time: i32,
    pub start_entry: &'a str,
    pub running: i32,
}
