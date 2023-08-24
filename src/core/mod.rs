mod buffer;
mod storage;
mod table;

use crate::query::ast::QueryStatement;

use self::buffer::BufferPool;

pub struct Executer<'a> {
    storage_path: &'a str,
    buffer: BufferPool,
}

impl<'a> Executer<'a> {
    pub fn new(storage_path: &'a str) -> Self {
        let raw = std::fs::read(storage_path).unwrap_or(vec![]);
        let buffer = BufferPool::from_bytes(&raw);
        Self {
            buffer,
            storage_path,
        }
    }

    /// return value means whether to continue the repl
    /// if return false, then exits
    pub fn execute(&mut self, query: Vec<QueryStatement>) -> bool {
        for stmt in query.iter() {
            match stmt {
                QueryStatement::Select(is_all, columns) => {
                    if *is_all {
                        for (key, value) in self.buffer.body.iter() {
                            println!("{}: {}", key, value);
                        }
                    } else {
                        for column in columns.iter() {
                            match self.buffer.body.get(column) {
                                Some(value) => println!("{}: {}", column, value),
                                None => println!("{}: not found", column),
                            }
                        }
                    }
                }
                // QueryStatement::Update(value) => {
                //     for (key, value) in value.iter() {
                //         self.buffer.body.insert(key.clone(), *value);
                //     }
                //     std::fs::write(self.storage_path, self.buffer.as_bytes()).unwrap();
                // }
                QueryStatement::Exit => {
                    println!("bye!");
                    return false;
                }
                _ => {
                    todo!()
                }
            }
        }
        true
    }
}
