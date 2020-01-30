pub mod entity;
pub mod error;
pub mod parser;

use entity::GAuthor;
use error::GError;

static BASE_API_URL: &'static str = "http://www.goodreads.com/";
static CLIENT_KEY: &'static str = "bTDP9FC0ax7CKuOQxHnIoQ";

pub struct GreadsClient {
    key: &'static str,
    hclient: reqwest::Client,
}

impl GreadsClient {
    pub fn new() -> GreadsClient {
        GreadsClient {
            key: CLIENT_KEY,
            hclient: reqwest::Client::new(),
        }
    }

    pub async fn author_show(&self, author_id: u32) -> Result<GAuthor, GError> {
        let url_base = format!("{}/author/show.xml", BASE_API_URL);
        let mut url = url::Url::parse(&url_base).unwrap();
        url.query_pairs_mut().append_pair("key", self.key);
        url.query_pairs_mut()
            .append_pair("id", &author_id.to_string());

        let result = self.hclient.get(url.as_str()).send().await?.text().await?;
        parser::parse_author(&result).map_err(GError::ParsingError)
    }
}
