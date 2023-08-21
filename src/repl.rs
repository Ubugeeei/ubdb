use std::io::Write;

use crate::{
    core::Executer,
    query::{lex::Lexer, parser::Parser},
};

const STORAGE_PATH: &str = "storage.db";

pub fn start() {
    let mut executer = Executer::new(STORAGE_PATH);

    loop {
        // prompt
        print!("> ");
        std::io::stdout().flush().unwrap();

        // receive query from stdin
        let mut query_raw = String::new();
        std::io::stdin().read_line(&mut query_raw).unwrap();

        // parsing
        let lexer = Lexer::new(query_raw);
        let mut parser = Parser::new(lexer);
        let query = parser.parse();

        // execution
        match query {
            Ok(query_stmts) => {
                let is_continue = executer.execute(query_stmts);
                if !is_continue {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
    }
}
