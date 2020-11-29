use std::io::{self, Read};

fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Pass in HTML via STDIN");
    buffer
}

fn main() {
    let html = read_input();
    let md = pdfparse::parse_html(&html);
    println!("{}", md);
}
