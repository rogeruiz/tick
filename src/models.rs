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
