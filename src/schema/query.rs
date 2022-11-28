use async_graphql::Object;

use crate::schema::book::Book;

pub struct Query;

#[Object]
impl Query {
    async fn name(&self) -> &'static str {
        "Zulhilmi"
    }

    async fn book(&self) -> Book {
        Book::new(String::from("Lord of the Rings"), String::from("Tolkien"))
    }
}
