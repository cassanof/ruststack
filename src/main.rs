use rustystack::{assembler::Assembler, parser::ASTParser, cpu::CPU};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed = ASTParser::parse_file(&args[1]);
    match parsed {
        Ok(ast) => {
            println!("--------- SUCCESFULLY PARSED ---------");
            // for node in ast {
            // println!("{:?}", node);
            // }
            match Assembler::assemble(ast) {
                Ok(mem) => {
                    println!("--------- SUCCESFULLY ASSEMBLED ---------");
                    let cpu = CPU::new(mem);
                    cpu.run().unwrap();
                }
                Err(e) => {
                    println!("--------- ERROR IN ASSEMBLING ---------");
                    println!("{}", e);
                }
            }
        }
        Err(e) => {
            println!("--------- ERROR IN PARSING ---------");
            println!("{}", e);
        }
    }
}
