#[derive(Debug)]
pub struct GAuthor {
    pub id: u32,
    pub name: String,
    pub gender: Option<String>,
    pub hometown: Option<String>,
    pub born_at: Option<String>,
    pub died_at: Option<String>,
}
