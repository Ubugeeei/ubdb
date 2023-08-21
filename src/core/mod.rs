use crate::query::ast::QueryStatement;

pub struct Executer<'a> {
    storage_path: &'a str,
    buffer: Option<i32>,
}

impl<'a> Executer<'a> {
    pub fn new(storage_path: &'a str) -> Self {
        let buffer = match std::fs::read_to_string(storage_path)
            .unwrap_or_else(|_| String::from("0"))
            .parse::<i32>()
        {
            Ok(value) => Some(value),
            Err(_) => None,
        };

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
                QueryStatement::Get => match self.buffer {
                    Some(value) => println!("{}", value),
                    None => println!("empty"),
                },
                QueryStatement::Set(value) => {
                    self.buffer = Some(*value);
                    std::fs::write(self.storage_path, value.to_string()).unwrap();
                }
                QueryStatement::Exit => {
                    println!("bye!");
                    return false;
                }
            }
        }
        true
    }
}
