use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Book {
    id: i32,
    title: String,
    author: String,
    published: bool,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, con: &mut PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(con)
            .expect("Error loading book {id}")
    }

    pub fn all(con: &mut PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(con)
            .expect("Error loading books.")
    }

    pub fn update_by_id(id: i32, con: &mut PgConnection, new_book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            author,
            title,
            published,
        } = new_book;
        diesel::update(all_books.find(id))
            .set((a.eq(author), t.eq(title), p.eq(published)))
            .get_result::<Book>(con)
            .is_ok()
    }

    pub fn insert(book: NewBook, con: &mut PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(con)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, con: &mut PgConnection) -> bool {
        if Book::show(id, con).is_empty() {
            return false;
        }
        diesel::delete(all_books.find(id)).execute(con).is_ok()
    }

    pub fn all_by_author(author: String, con: &mut PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .order(books::id.desc())
            .load::<Book>(con)
            .expect("Error loading books")
    }
}
