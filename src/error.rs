#[derive(Debug)]
pub enum GError {
    RequestError(reqwest::Error),
    ParsingError(roxmltree::Error),
    NotFound,
}

impl From<reqwest::Error> for GError {
    fn from(e: reqwest::Error) -> Self {
        GError::RequestError(e)
    }
}

impl From<roxmltree::Error> for GError {
    fn from(e: roxmltree::Error) -> Self {
        GError::ParsingError(e)
    }
}
