use std::collections::BTreeMap;

use crate::query::ast::QueryStatement;

pub struct Executer<'a> {
    storage_path: &'a str,
    buffer: BTreeMap<String, i32>,
}

impl<'a> Executer<'a> {
    pub fn new(storage_path: &'a str) -> Self {
        // TODO: load
        // let buffer = match std::fs::read_to_string(storage_path)
        //     .unwrap_or_else(|_| String::from("0"))
        //     .parse::<i32>()
        // {
        //     Ok(value) => Some(value),
        //     Err(_) => None,
        // };

        Self {
            buffer: BTreeMap::new(),
            storage_path,
        }
    }

    /// return value means whether to continue the repl
    /// if return false, then exits
    pub fn execute(&mut self, query: Vec<QueryStatement>) -> bool {
        for stmt in query.iter() {
            match stmt {
                QueryStatement::Select { is_all, columns } => {
                    if *is_all {
                        for (key, value) in self.buffer.iter() {
                            println!("{}: {}", key, value);
                        }
                    } else {
                        for column in columns.iter() {
                            match self.buffer.get(column) {
                                Some(value) => println!("{}: {}", column, value),
                                None => println!("{}: not found", column),
                            }
                        }
                    }
                }
                QueryStatement::Set(value) => {
                    for (key, value) in value.iter() {
                        self.buffer.insert(key.clone(), *value);
                    }
                    // TODO: sync
                    // std::fs::write(self.storage_path, value.to_string()).unwrap();
                }
                QueryStatement::Exit => {
                    println!("bye!");
                    return false;
                }
                _ => todo!(),
            }
        }
        true
    }
}
