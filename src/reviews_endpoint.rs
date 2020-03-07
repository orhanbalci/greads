use super::entity::GReview;
use super::error::GError;
use super::GreadsClient;
use futures::future::FutureExt;

pub struct ReviewsEndpoint<'a> {
    pub client: &'a GreadsClient,
    show_endpoint: &'static str,
}

impl<'a> ReviewsEndpoint<'a> {
    pub fn new(client: &'a GreadsClient) -> ReviewsEndpoint {
        ReviewsEndpoint {
            client: client,
            show_endpoint: "review/show",
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
}
