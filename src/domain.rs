use chrono;

use super::schema::books;

#[derive(Queryable, Identifiable, AsChangeset, Clone)]
pub struct Book {
    pub id : i32,
    pub title : Option<String>,
    pub save_date : Option<chrono::NaiveDateTime>, 
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub title : Option<&'a str>,
    pub save_date : Option<&'a chrono::NaiveDateTime>, 
}
