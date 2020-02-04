use super::entity::GBook;
use super::error::GError;
use super::GreadsClient;
use futures::future::FutureExt;

pub struct BooksEndpoint<'a> {
    pub client: &'a GreadsClient,
    search_endpoint: &'static str,
    author_endpoint: &'static str,
}

impl<'a> BooksEndpoint<'a> {
    pub fn new(client: &'a GreadsClient) -> Self {
        BooksEndpoint {
            client: client,
            search_endpoint: "search/index.xml",
            author_endpoint: "author/show.xml",
        }
    }
    pub async fn get_by_isbn(&self, query: &str, page: u32) -> Result<Vec<GBook>, GError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.search_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", self.client.key);
        url.query_pairs_mut().append_pair("q", query);
        url.query_pairs_mut().append_pair("page", &page.to_string());
        url.query_pairs_mut().append_pair("search", "isbn");
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
