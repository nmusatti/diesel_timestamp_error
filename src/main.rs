extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use std::env;

use diesel::{ Connection, LoadDsl, result, sqlite };
use dotenv::dotenv;

pub mod domain {
	use chrono::naive;

	#[derive(Queryable)]
	pub struct Book {
	    id : i32,
	    title : Option<String>,
	    save_date : Option<naive::NaiveDateTime>, 
	}
}

pub mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

fn main() {
    use self::schema::books::dsl::*;
	let database_url = env::var("DATABASE_URL").unwrap();
	let conn = sqlite::SqliteConnection::establish(&database_url).unwrap();
    let res = books.load::<domain::Book>(&conn);
}
