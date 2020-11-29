use select::document::Document;
use select::node::Node;
use select::predicate::{Any, Class, Name, Predicate, Text};

/// Find first node occurring _after_ the passed node
/// matching the given predicate. Limits search to
/// 10 subsequent nodes.
fn find_nearest_subsequent_matching<'a>(
    node: &Node,
    predicate: &dyn Predicate,
    document: &'a Document,
) -> Option<Node<'a>> {
    let index = node.raw().index;

    for i in (index + 1)..(index + 12) {
        if let Some(node) = document.nth(i) {
            if predicate.matches(&node) {
                return Some(node);
            }
        }
    }

    None
}

fn parse_node_recursively(node: &Node) -> Vec<String> {
    let mut italic = false;

    if Class("_bc58d0099348bf91").matches(&node) {
        italic = true;
    }

    let mut results = if Text.matches(&node) {
        if node.text().trim().len() > 0 {
            vec![node.text().trim().to_owned()]
        } else {
            vec![]
        }
    } else {
        node.children()
            .map(|n| parse_node_recursively(&n))
            .flatten()
            .collect()
    };

    if italic {
        results.insert(0, " *".to_owned());
        results.push("* ".to_owned());
    }

    results
}

fn parse_paragraph(node: &Node) -> Vec<String> {
    let children = node.find(Class("highlight"));
    children
        .map(|n| parse_node_recursively(&n))
        .flatten()
        .collect()
}

fn parse_note(node: &Node) -> Vec<String> {
    let children = node.find(Name("span"));
    children
        .map(|n| parse_node_recursively(&n))
        .flatten()
        .collect()
}

fn parse_rectangle(node: &Node, document: &Document) -> String {
    let page_number = Class("page");
    let page_number = find_nearest_subsequent_matching(&node, &page_number, &document);

    match page_number {
        Some(node) => format!(
            "**SPACE TO PASTE IMAGE FROM {}**",
            node.text().to_uppercase()
        ),
        None => "**SPACE TO PASTE IMAGE**".to_owned(),
    }
}

fn parse_node(node: Node, document: &Document) -> Option<String> {
    if Name("p").matches(&node) {
        let text = parse_paragraph(&node);
        if text.len() > 0 {
            let text = text.join(" ");
            let text = format!("> {}", text);
            Some(text)
        } else {
            None
        }
    } else if Text.matches(&node) && node.text() == "Rectangle" {
        Some(parse_rectangle(&node, &document))
    } else if Class("with-border").matches(&node) {
        let text = parse_note(&node);
        let text = text.join("");
        Some(text)
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

    let text: Vec<String> = document
        .find(Any)
        .flat_map(|node| parse_node(node, &document))
        .collect();

    // if let Some(title) = get_title(&document) {
    //     text.insert(0, title);
    // }

    text.join("\n\n")
}
