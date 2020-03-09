use std::string::ToString;

pub enum Order {
    Asc,
    Desc,
}

impl ToString for Order {
    fn to_string(&self) -> String {
        match self {
            Order::Asc => "a".to_owned(),
            Order::Desc => "d".to_owned(),
        }
    }
}
