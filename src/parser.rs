use super::entity::GAuthor;
pub fn parse_author(author_content: &str) -> Result<GAuthor, roxmltree::Error> {
    let doc = roxmltree::Document::parse(author_content)?;
    let id = get_node_text(&doc, "id");
    let name = get_node_text(&doc, "name");
    let hometown = get_node_text(&doc, "hometown");
    let gender = get_node_text(&doc, "gender");
    let born_at = get_node_text(&doc, "born_at");
    let died_at = get_node_text(&doc, "died_at");
    Ok(GAuthor {
        id: id.unwrap().parse::<u32>().unwrap(),
        name: name.unwrap().to_owned(),
        hometown,
        gender,
        born_at,
        died_at,
    })
}

fn get_node_text(doc: &roxmltree::Document, tag_name: &str) -> Option<String> {
    if let Some(a) = doc
        .descendants()
        .find(|node| node.is_element() && node.tag_name().name() == tag_name)
    {
        a.text().map(|s| s.to_owned())
    } else {
        None
    }
}
