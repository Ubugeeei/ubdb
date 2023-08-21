use std::io::Write;
mod query;

enum Query {
    Set(i32),
    Get,
    Exit,
}

const STORAGE_PATH: &str = "storage.db";

fn main() {
    // init from storage
    let mut buffer = match std::fs::read_to_string(STORAGE_PATH)
        .unwrap_or_else(|_| String::from("0"))
        .parse::<i32>()
    {
        Ok(value) => Some(value),
        Err(_) => None,
    };

    // repl
    loop {
        // prompt
        print!("> ");
        std::io::stdout().flush().unwrap();

        // receive query from stdin
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();

        // parse
        let query = match query.trim() {
            "get" => Query::Get,
            "exit" => Query::Exit,
            set => {
                let set = set.trim_start_matches("set ");
                Query::Set(set.parse().unwrap())
            }
        };

        // process
        match query {
            Query::Set(value) => {
                buffer = Some(value);
                std::fs::write(STORAGE_PATH, value.to_string()).unwrap();
            }
            Query::Get => match buffer {
                Some(value) => println!("{}", value),
                None => println!("empty"),
            },
            Query::Exit => {
                println!("bye!");
                break;
            }
        }
    }
}
