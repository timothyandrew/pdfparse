use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Any, Class, Name, Text, And, Not};

/// Find first node occurring _after_ the passed node
/// matching the given predicate. Limits search to 
/// 10 subsequent nodes.
fn find_nearest_subsequent_matching<'a>(node: &Node, predicate: &dyn Predicate, document: &'a Document) -> Option<Node<'a>> {
    let index = node.raw().index;

    for i in (index + 1)..(index + 12) {
        if let Some(node) = document.nth(i) {
            if predicate.matches(&node) {
                return Some(node)
            }
        }
    }

    None
}

fn parse_paragraph(node: &Node) -> Option<String> {
    let text = node.find(Class("highlight"));
    let text = text.map(|node| node.text().trim().to_owned());
    let text = text.filter(|s| s.len() > 0 || s != "…");
    let text: Vec<String> = text.collect();

    if text.len() == 0 {
        None
    } else {
        let text = text.join(" ");
        let text = text.trim();
        let text = format!("> {}", text);
        Some(text)
    }
}

fn parse_note(node: &Node) -> Option<String> {
    if let Some(c) = node.attr("class") {
        if c == "" {
            None
        } else {
            Some(node.text())
        }
    } else {
        None
    }
}

fn parse_rectangle(node: &Node, document: &Document) -> String {
    let page_number = Class("page");
    let page_number = find_nearest_subsequent_matching(&node, &page_number, &document);

    match page_number {
        Some(node) => format!("**SPACE TO PASTE IMAGE FROM {}**", node.text().to_uppercase()),
        None => "**SPACE TO PASTE IMAGE**".to_owned()
    }
}

fn parse_node(node: Node, document: &Document) -> Option<String> {
    if Name("p").matches(&node) {
        parse_paragraph(&node)
    } else if Text.matches(&node) && node.text() == "Rectangle" {
        Some(parse_rectangle(&node, &document))
    } else if And(Name("span"), Not(Class("highlight"))).matches(&node) {
        parse_note(&node)
    } else {
        None
    }
}

fn get_title(document: &Document) -> Option<String> {
    let title = document.find(Name("title")).collect::<Vec<_>>();
    let title = title.get(0);
    title.map(|node| {
        let text = node.text();
        let text = text.replace("annotated pages", "");
        let text = text.trim();
        format!("# {}", text)
    })
}

pub fn parse_html(html: &str) -> String {
    let document = Document::from(html);

    let mut text: Vec<String> = document.find(Any).flat_map(|node| parse_node(node, &document)).collect();

    if let Some(title) = get_title(&document) {
        text.insert(0, title);
    }

    text.join("\n\n")
}