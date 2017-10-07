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

I'm trying to insert records into it with the following struct:

```rust
    pub mod domain {
        use chrono;
    
        use super::schema::books;
        
        #[derive(Insertable)]
        #[table_name="books"]
        pub struct NewBook<'a> {
            pub title : Option<&'a str>,
            pub save_date : Option<&'a chrono::NaiveDateTime>, 
        }
    
    }
```

However when I try to compile it I get several error messages complaining about unsatisfied trait
bounds such as the following:

    error[E0277]: the trait bound `chrono::NaiveDateTime: diesel::Expression` is not satisfied
      --> src/main.rs:30:11
       |
    30 |    #[derive(Insertable)]
       |             ^^^^^^^^^^ the trait `diesel::Expression` is not implemented for `chrono::NaiveDateTime`
       |
       = note: required because of the requirements on the impl of `diesel::Expression` for `&'a chrono::NaiveDateTime`
       = note: this error originates in a macro outside of the current crate
    
    error[E0277]: the trait bound `&'insert domain::NewBook<'a>: diesel::Insertable<schema::__diesel_infer_schema::infer_books::books::table, DB>` is not satisfied
      --> src/main.rs:30:11
       |
    30 |    #[derive(Insertable)]
       |             ^^^^^^^^^^ the trait `diesel::Insertable<schema::__diesel_infer_schema::infer_books::books::table, DB>` is not implemented for `&'insert domain::NewBook<'a>`
       |
       = help: the following implementations were found:
                 <&'insert domain::NewBook<'a> as diesel::Insertable<schema::__diesel_infer_schema::infer_books::books::table, DB>>
       = note: required by `diesel::Insertable`
       = note: this error originates in a macro outside of the current crate

Note that this works with diesel 0.14, but breaks with 0.15 and 0.16.
