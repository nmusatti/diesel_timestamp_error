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

pub mod schema {
	infer_schema!("dotenv:DATABASE_URL");
}

pub mod domain {
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
}

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
    			if let Some(sd) = b.save_date {
		    		println!("{}", sd);
    			}
    		}
    	},
    	Err(e) => println!("{}", e)
    }
}
