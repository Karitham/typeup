use typeup::lexer;

fn main() {
    let content = std::fs::read_to_string("../example/example.tu").expect("can't read file");
    let document = lexer::tokenize(content);

    println!("{:#?}", document)
}
