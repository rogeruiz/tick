use super::schema::timers;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Timer {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub start_entry: Option<String>,
    pub end_entry: Option<String>,
}

#[derive(Insertable)]
#[table_name="timers"]
pub struct NewTimer<'a> {
    pub name: &'a str,
    pub start_time: &'a str,
    pub start_entry: &'a str,
}
