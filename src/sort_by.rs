use std::string::ToString;

pub enum SortBy {
    Title,
    Author,
    Cover,
    Rating,
    YearPub,
    DatePub,
    DatePubEdition,
    DateStarted,
    DateRead,
    DateUpdated,
    DateAdded,
    Recommender,
    AvgRating,
    NumRatings,
    Review,
    ReadCount,
    Votes,
    Random,
    Comments,
    Notes,
    Isbn,
    Isbn13,
    Asin,
    NumPages,
    Format,
    Position,
    Shelves,
    Owned,
    DatePurchased,
    PurchaseLocation,
    Condition,
}

impl ToString for SortBy {
    fn to_string(&self) -> String {
        match self {
            SortBy::Title => "title".to_owned(),
            SortBy::Author => "author".to_owned(),
            SortBy::Cover => "cover".to_owned(),
            SortBy::Rating => "rating".to_owned(),
            SortBy::YearPub => "year_pub".to_owned(),
            SortBy::DatePub => "date_pub".to_owned(),
            SortBy::DatePubEdition => "date_pub_edition".to_owned(),
            SortBy::DateStarted => "date_started".to_owned(),
            SortBy::DateRead => "date_read".to_owned(),
            SortBy::DateUpdated => "date_updated".to_owned(),
            SortBy::DateAdded => "date_added".to_owned(),
            SortBy::Recommender => "recommender".to_owned(),
            SortBy::AvgRating => "avg_rating".to_owned(),
            SortBy::NumRatings => "num_ratings".to_owned(),
            SortBy::Review => "review".to_owned(),
            SortBy::ReadCount => "read_count".to_owned(),
            SortBy::Votes => "votes".to_owned(),
            SortBy::Random => "random".to_owned(),
            SortBy::Comments => "comments".to_owned(),
            SortBy::Notes => "notes".to_owned(),
            SortBy::Isbn => "isbn".to_owned(),
            SortBy::Isbn13 => "isbn13".to_owned(),
            SortBy::Asin => "asin".to_owned(),
            SortBy::NumPages => "num_pages".to_owned(),
            SortBy::Format => "format".to_owned(),
            SortBy::Position => "position".to_owned(),
            SortBy::Shelves => "shelves".to_owned(),
            SortBy::Owned => "owned".to_owned(),
            SortBy::DatePurchased => "date_purchased".to_owned(),
            SortBy::PurchaseLocation => "purchase_location".to_owned(),
            SortBy::Condition => "condition".to_owned(),
        }
    }
}
