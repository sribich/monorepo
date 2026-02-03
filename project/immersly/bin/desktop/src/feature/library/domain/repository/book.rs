use async_trait::async_trait;

use crate::feature::library::domain::aggregate::book::Book;

macro_rules! repository {
    ($name:ident) => {
        pub trait ${ concat($name, Repository) }: Send + Sync {
            fn reader(&self) -> std::sync::Arc<dyn ${concat($name, Reader)}>;

            fn writer(&self) -> std::sync::Arc<dyn ${concat($name, Writer)}>;
        }
    };
}

repository!(Book);

#[async_trait]
pub trait BookReader: Send + Sync {
    async fn find_all(&self) -> Vec<Book>;
}

#[async_trait]
pub trait BookWriter: Send + Sync {
    async fn create(&self, book: &Book);
}
