extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use std::env;

use chrono::prelude::*;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod schema;
pub mod domain;

fn main() {
	dotenv().ok();
    use self::schema::books::dsl::*;
	let database_url = env::var("DATABASE_URL").unwrap();
	let conn = diesel::sqlite::SqliteConnection::establish(&database_url).unwrap();
	let t = "The Lord of the Rings";
	let sd = NaiveDateTime::from_timestamp(Local::now().timestamp(), 0);
    let nb = domain::NewBook{
        title : Some(&t),
        save_date : Some(&sd)
    };
    diesel::insert(&nb).into(books).execute(&conn).unwrap();
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
