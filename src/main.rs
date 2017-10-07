extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use std::env;

use diesel::{ Connection, LoadDsl, sqlite };
use dotenv::dotenv;

pub mod domain {
	use chrono::naive;

	#[derive(Queryable)]
	pub struct Book {
	    pub id : i32,
	    pub title : Option<String>,
	    pub save_date : Option<naive::NaiveDateTime>, 
	}
}

pub mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

fn main() {
	dotenv().ok();
    use self::schema::books::dsl::*;
	let database_url = env::var("DATABASE_URL").unwrap();
	let conn = sqlite::SqliteConnection::establish(&database_url).unwrap();
    match books.load::<domain::Book>(&conn) {
    	Ok(vb) => {
    		for b in vb {
    			if let Some(t) = b.title {
		    		println!("{}", t);
    			}			
    		}
    	},
    	Err(e) => println!("{}", e)
    }
}
