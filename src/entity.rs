#[derive(Debug)]
pub struct GAuthor {
    pub id: u32,
    pub name: String,
    pub gender: Option<String>,
    pub hometown: Option<String>,
    pub born_at: Option<String>,
    pub died_at: Option<String>,
}

#[derive(Debug)]
pub struct GBook {
    pub id: u32,
    pub isbn: String,
    pub title: String,
    pub num_pages: Option<u32>,
    pub average_rating: Option<f32>,
    pub description: Option<String>,
}
