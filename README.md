This project displays a problem I'm having with Diesel, the Rust ORM. I'm trying to use its
schema inference capability to work with a SQLite table that contains a nullable timestamp column.

The ``books`` table is defined as follows:

```sql
    create table books (
        id integer not null primary key,
        title varchar null,
        save_date timestamp null 
    )
```

I'm trying to query it with the following struct:

```rust
    pub mod domain {
        use chrono::naive;
    
        #[derive(Queryable)]
        pub struct Book {
            id : i32,
            title : Option<String>,
            save_date : Option<naive::NaiveDateTime>, 
        }
    }
```

However when I try to actually perform the query, as in:

```rust
    fn main() {
        use self::schema::books::dsl::*;
        let database_url = env::var("DATABASE_URL").unwrap();
        let conn = sqlite::SqliteConnection::establish(&database_url).unwrap();
        let res = books.load::<domain::Book>(&conn);
    }
```

I get the following error message:

    error[E0277]: the trait bound `std::option::Option<chrono::NaiveDateTime>: diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Timestamp>, _>` is not satisfied
      --> src/main.rs:32:21
       |
    32 |     let res = books.load::<domain::Book>(&conn);
       |                     ^^^^ the trait `diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Timestamp>, _>` is not implemented for `std::option::Option<chrono::NaiveDateTime>`
       |
       = help: the following implementations were found:
                 <std::option::Option<chrono::naive::date::NaiveDate> as diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Date>, DB>>
                 <std::option::Option<chrono::naive::time::NaiveTime> as diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Time>, DB>>
                 <std::option::Option<chrono::naive::datetime::NaiveDateTime> as diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Timestamp>, DB>>
                 <std::option::Option<bool> as diesel::types::FromSqlRow<diesel::types::Nullable<diesel::types::Bool>, DB>>
               and 26 others
       = note: required because of the requirements on the impl of `diesel::types::FromSqlRow<(diesel::types::Integer, diesel::types::Nullable<diesel::types::Text>, diesel::types::Nullable<diesel::types::Timestamp>), _>` for `(i32, std::option::Option<std::string::String>, std::option::Option<chrono::NaiveDateTime>)`
       = note: required because of the requirements on the impl of `diesel::Queryable<(diesel::types::Integer, diesel::types::Nullable<diesel::types::Text>, diesel::types::Nullable<diesel::types::Timestamp>), _>` for `domain::Book`

What I don't understand is why the implementation for ``chrono::naive::datetime::NaiveDateTime``
isn't picked up and what I should do to make it happen.
