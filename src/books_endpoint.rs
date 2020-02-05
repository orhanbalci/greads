use super::entity::GBook;
use super::error::GError;
use super::GreadsClient;
use futures::future::FutureExt;

pub struct BooksEndpoint<'a> {
    pub client: &'a GreadsClient,
    isbn_endpoint: &'static str,
    bookid_endpoint: &'static str,
    title_endpoint: &'static str,
    author_endpoint: &'static str,
}

impl<'a> BooksEndpoint<'a> {
    pub fn new(client: &'a GreadsClient) -> Self {
        BooksEndpoint {
            client: client,
            isbn_endpoint: "book/isbn/",
            bookid_endpoint: "book/show/",
            title_endpoint: "book/title.xml",
            author_endpoint: "author/show.xml",
        }
    }

    pub async fn get_by_title<T>(&self, title: T, author: T) -> Result<GBook, GError>
    where
        T: ToString,
    {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.title_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", self.client.key);
        url.query_pairs_mut()
            .append_pair("title", &title.to_string());
        url.query_pairs_mut()
            .append_pair("author", &author.to_string());
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_get_by_isbn(&k.unwrap()).map_err(GError::ParsingError))
            .await;
        result
    }

    pub async fn get_by_isbn(&self, isbn: &str) -> Result<GBook, GError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}/{}.xml",
            self.client.base_api_url, self.isbn_endpoint, isbn
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", self.client.key);
        url.query_pairs_mut().append_pair("isbn", isbn);
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_get_by_isbn(&k.unwrap()).map_err(GError::ParsingError))
            .await;
        result
    }

    pub async fn get_by_book_id<T>(&self, book_id: T) -> Result<GBook, GError>
    where
        T: ToString,
    {
        let mut url = url::Url::parse(&format!(
            "{}/{}/{}.xml",
            self.client.base_api_url,
            self.bookid_endpoint,
            book_id.to_string()
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", self.client.key);
        url.query_pairs_mut()
            .append_pair("bookId", &book_id.to_string());
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_get_by_isbn(&k.unwrap()).map_err(GError::ParsingError))
            .await;
        result
    }

    pub async fn get_by_author_id(
        &self,
        author_id: u32,
        page_number: u32,
    ) -> Result<Vec<GBook>, GError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.author_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", self.client.key);
        url.query_pairs_mut()
            .append_pair("id", &author_id.to_string());
        url.query_pairs_mut()
            .append_pair("page", &page_number.to_string());

        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_books(&k.unwrap()).map_err(GError::ParsingError))
            .await;
        result
    }
}
