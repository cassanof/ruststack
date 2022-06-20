use rustystack::parser::ASTParser;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed = ASTParser::parse_file(&args[1]);
    match parsed {
        Ok(ast) => {
            for node in ast {
                println!("{:?}", node);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
