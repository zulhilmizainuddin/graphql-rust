use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Book {
    title: String,
    author: String,
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        Book {
            title,
            author,
        }
    }
}
