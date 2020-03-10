use super::entity::GReview;
use super::error::GError;
use super::order::Order;
use super::sort_by::SortBy;
use super::GreadsClient;

use chrono::NaiveDate;
use futures::future::FutureExt;

pub struct ReviewsEndpoint<'a> {
    pub client: &'a GreadsClient,
    show_endpoint: &'static str,
    user_book_endpoint: &'static str,
    recent_reviews_endpoint: &'static str,
    list_endpoint: &'static str,
    create_endpoint: &'static str,
}

impl<'a> ReviewsEndpoint<'a> {
    pub fn new(client: &'a GreadsClient) -> ReviewsEndpoint {
        ReviewsEndpoint {
            client: client,
            show_endpoint: "review/show",
            user_book_endpoint: "review/show_by_user_and_book",
            recent_reviews_endpoint: "review/recent_reviews",
            list_endpoint: "review/list",
            create_endpoint: "review.xml",
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

    pub async fn get_user_reviews<T, Q>(
        &self,
        user_id: u64,
        shelf_name: Option<T>,
        sort_by: Option<SortBy>,
        search_query: Option<Q>,
        order_by: Option<Order>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<GReview>, GError>
    where
        T: ToString,
        Q: ToString,
    {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.list_endpoint
        ))
        .unwrap();

        url.query_pairs_mut().append_pair("key", &self.client.key);
        url.query_pairs_mut()
            .append_pair("user_id", &user_id.to_string());
        if let Some(shelf) = shelf_name {
            url.query_pairs_mut()
                .append_pair("shelf", &shelf.to_string());
        }
        if let Some(sort) = sort_by {
            url.query_pairs_mut().append_pair("sort", &sort.to_string());
        }
        if let Some(search) = search_query {
            url.query_pairs_mut()
                .append_pair("search[query]", &search.to_string());
        }
        if let Some(order) = order_by {
            url.query_pairs_mut()
                .append_pair("order", &order.to_string());
        }
        if let Some(page) = page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }
        if let Some(page_size) = page_size {
            url.query_pairs_mut()
                .append_pair("per_page", &page_size.to_string());
        }

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

    pub async fn create_review<T, S>(
        &self,
        book_id: u64,
        review: Option<T>,
        rating: Option<u32>,
        date_read: Option<NaiveDate>,
        shelf_name: Option<S>,
    ) -> Result<u64, GError>
    where
        T: Into<String>,
        S: Into<String>,
    {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.client.base_api_url, self.create_endpoint
        ))
        .unwrap();
        url.query_pairs_mut().append_pair("key", &self.client.key);
        url.query_pairs_mut()
            .append_pair("book_id", &book_id.to_string());
        if let Some(review_text) = review {
            url.query_pairs_mut()
                .append_pair("review[text]", &review_text.into());
        }

        if let Some(rating) = rating {
            url.query_pairs_mut()
                .append_pair("review[rating]", &rating.to_string());
        }

        if let Some(date_read) = date_read {
            url.query_pairs_mut()
                .append_pair("review[read_at]", &date_read.format("%F").to_string());
        }

        if let Some(shelf_name) = shelf_name {
            url.query_pairs_mut()
                .append_pair("shelf", &shelf_name.into());
        }

        let result = self
            .client
            .hclient
            .post(url.as_str())
            .send()
            .then(|r| r.unwrap().text())
            .map(|k| super::parser::parse_create_review(&k.unwrap()).map_err(GError::ParsingError))
            .await;
        result
    }
}
