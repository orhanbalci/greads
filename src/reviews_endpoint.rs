use super::entity::GReview;
use super::error::GError;
use super::GreadsClient;
use futures::future::FutureExt;

pub struct ReviewsEndpoint<'a> {
    pub client: &'a GreadsClient,
    show_endpoint: &'static str,
    user_book_endpoint: &'static str,
    recent_reviews_endpoint : &'static str,
}

impl<'a> ReviewsEndpoint<'a> {
    pub fn new(client: &'a GreadsClient) -> ReviewsEndpoint {
        ReviewsEndpoint {
            client: client,
            show_endpoint: "review/show",
            user_book_endpoint: "review/show_by_user_and_book",
            recent_reviews_endpoint: "review/recent_reviews",
        }
    }

    pub async fn get_by_id(&self, id: u64, page: u32) -> Result<GReview, GError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.show_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", &self.client.key);
        url.query_pairs_mut().append_pair("id", &id.to_string());
        url.query_pairs_mut().append_pair("page", &page.to_string());
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_review(&k.unwrap()).map_err(GError::ParsingError))
            .await;

        result
    }

    pub async fn get_by_user_id_and_book_id(
        &self,
        user_id: u64,
        book_id: u64,
        find_on_different_edition: bool,
    ) -> Result<GReview, GError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.user_book_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", &self.client.key);
        url.query_pairs_mut()
            .append_pair("user_id", &user_id.to_string());
        url.query_pairs_mut()
            .append_pair("book_id", &book_id.to_string());
        url.query_pairs_mut().append_pair(
            "include_review_on_work",
            &find_on_different_edition.to_string(),
        );
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_review(&k.unwrap()).map_err(GError::ParsingError))
            .await;

        result
    }

    pub async fn get_recent_reviews(&self) -> Result<Vec<GReview>, GError> {
         let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.recent_reviews_endpoint
        ))
        .unwrap();
        let result = self
            .client
            .hclient
            .get(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_reviews(&k.unwrap()).map_err(GError::ParsingError))
            .await;

        result
    }
}
