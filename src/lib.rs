pub mod books_endpoint;
pub mod entity;
pub mod error;
pub mod parser;
pub mod reviews_endpoint;

use books_endpoint::BooksEndpoint;
use entity::GAuthor;
use error::GError;
use oauth1::Token;
use reviews_endpoint::ReviewsEndpoint;

static BASE_API_URL: &'static str = "https://www.goodreads.com/";

pub struct GreadsClient {
    key: String,
    secret: String,
    hclient: reqwest::Client,
    base_api_url: &'static str,
}

impl GreadsClient {
    pub fn new<K, S>(client_key: K, client_secret: S) -> GreadsClient
    where
        K: ToString,
        S: ToString,
    {
        GreadsClient {
            key: client_key.to_string(),
            secret: client_secret.to_string(),
            hclient: reqwest::Client::new(),
            base_api_url: BASE_API_URL,
        }
    }

    pub fn to_authorized<T>(&self, auth_token: T) -> AuthorizedGreadsClient
    where
        T: ToString,
    {
        AuthorizedGreadsClient::new_from(&self, auth_token)
    }

    pub async fn request_token(&self) -> Result<Option<String>, GError> {
        let url_base = format!("{}oauth/request_token", BASE_API_URL);
        let mut url = url::Url::parse(&url_base).unwrap();
        let result = self
            .hclient
            .get(url.as_str())
            .header(
                "Authorization",
                oauth1::authorize(
                    "GET",
                    url.as_str(),
                    &Token::new(&self.key, &self.secret),
                    None,
                    None,
                ),
            )
            .send()
            .await?
            .text()
            .await?;
        Ok(Some(result))
    }

    pub async fn authorize_token(
        &self,
        token: &str,
        token_key: &str,
    ) -> Result<Option<String>, GError> {
        println!("{}", token);
        let url_base = format!("{}oauth/access_token", BASE_API_URL);
        let mut url = url::Url::parse(&url_base).unwrap();
        let result = self
            .hclient
            .get(url.as_str())
            .header(
                "Authorization",
                oauth1::authorize(
                    "GET",
                    url.as_str(),
                    &Token::new(&self.key, &self.secret),
                    Some(&Token::new(token, token_key)),
                    None,
                ),
            )
            .send()
            .await?
            .text()
            .await?;
        Ok(Some(result))
    }

    pub fn request_authorization_url(&self, token_key: &str) -> String {
        format!("https://www.goodreads.com/oauth/authorize?{}", token_key)
    }

    pub fn books<'a>(&self) -> BooksEndpoint {
        BooksEndpoint::new(&self)
    }

    pub fn reviews<'a>(&self) -> ReviewsEndpoint {
        ReviewsEndpoint::new(&self)
    }

    pub async fn author_show(&self, author_id: u32) -> Result<GAuthor, GError> {
        let url_base = format!("{}/author/show.xml", BASE_API_URL);
        let mut url = url::Url::parse(&url_base).unwrap();
        url.query_pairs_mut().append_pair("key", &self.key);
        url.query_pairs_mut()
            .append_pair("id", &author_id.to_string());

        let result = self.hclient.get(url.as_str()).send().await?.text().await?;
        parser::parse_author(&result).map_err(GError::ParsingError)
    }
}

pub struct AuthorizedGreadsClient {
    key: String,
    secret: String,
    hclient: reqwest::Client,
    base_api_url: &'static str,
    authorized_token: String,
}

impl AuthorizedGreadsClient {
    pub fn new<K, S, T>(client_key: K, client_secret: S, auth_token: T) -> AuthorizedGreadsClient
    where
        K: ToString,
        S: ToString,
        T: ToString,
    {
        AuthorizedGreadsClient {
            key: client_key.to_string(),
            secret: client_secret.to_string(),
            hclient: reqwest::Client::new(),
            base_api_url: BASE_API_URL,
            authorized_token: auth_token.to_string(),
        }
    }

    pub fn new_from<T>(gl: &GreadsClient, auth_token: T) -> AuthorizedGreadsClient
    where
        T: ToString,
    {
        AuthorizedGreadsClient {
            key: gl.key.clone(),
            secret: gl.secret.clone(),
            hclient: reqwest::Client::new(),
            base_api_url: BASE_API_URL,
            authorized_token: auth_token.to_string(),
        }
    }
}
