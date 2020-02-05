use super::entity::{GAuthor, GBook};
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

pub fn parse_books(book_content: &str) -> Result<Vec<GBook>, roxmltree::Error> {
    println!("{}", book_content);
    let doc = roxmltree::Document::parse(book_content)?;
    let book_nodes = doc
        .descendants()
        .find(|nd| nd.is_element() && nd.tag_name().name() == "books");

    let result = book_nodes
        .unwrap()
        .children()
        .filter(|c| c.is_element() && c.tag_name().name() == "book")
        .map(|bb| parse_book(bb))
        .collect::<Vec<GBook>>();
    Ok(result)
}

fn parse_book(book_node: roxmltree::Node) -> GBook {
    let id = get_node_text_from_parent_node(book_node, "id");
    let title = get_node_text_from_parent_node(book_node, "title");
    let num_pages = get_node_text_from_parent_node(book_node, "num_pages");
    let average_rating = get_node_text_from_parent_node(book_node, "average_rating");
    let description = get_node_text_from_parent_node(book_node, "description");
    GBook {
        id: id.map(|i| i.parse().unwrap()).unwrap(),
        title: title.unwrap(),
        num_pages: num_pages.map(|np| np.parse().unwrap()),
        average_rating: average_rating.map(|ar| ar.parse().unwrap()),
        description: description,
    }
}

fn get_node_text_from_parent_node(parent_node: roxmltree::Node, tag_name: &str) -> Option<String> {
    if let Some(a) = parent_node
        .children()
        .find(|node| node.is_element() && node.tag_name().name() == tag_name)
    {
        a.text().map(|s| s.to_owned())
    } else {
        None
    }
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

pub fn parse_get_by_isbn(get_by_isbn_content: &str) -> Result<GBook, roxmltree::Error> {
    println!("{}", get_by_isbn_content);
    let doc = roxmltree::Document::parse(get_by_isbn_content)?;
    let book_node = doc
        .descendants()
        .find(|nd| nd.is_element() && nd.tag_name().name() == "book")
        .expect("Unable to find book node");
    Ok(parse_book(book_node))
}
