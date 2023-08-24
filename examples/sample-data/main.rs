use ubdb::core::{
    table::{DataType, Record, Table, Value},
    Executer,
};

const STORAGE_PATH: &str = "db";

fn main() {
    let executer = Executer::new(STORAGE_PATH.to_string());

    let user_table = Table::new(
        String::from("user"),
        vec![
            (String::from("id"), DataType::Int),
            (String::from("name"), DataType::VarChar(10)),
        ],
        vec![
            Record::new(vec![Value::Int(1), Value::VarChar(String::from("alice"))]),
            Record::new(vec![Value::Int(2), Value::VarChar(String::from("bob"))]),
            Record::new(vec![Value::Int(3), Value::VarChar(String::from("charlie"))]),
            Record::new(vec![Value::Int(4), Value::VarChar(String::from("david"))]),
            Record::new(vec![Value::Int(5), Value::VarChar(String::from("eve"))]),
            Record::new(vec![Value::Int(6), Value::VarChar(String::from("frank"))]),
            Record::new(vec![Value::Int(7), Value::VarChar(String::from("grace"))]),
            Record::new(vec![Value::Int(8), Value::VarChar(String::from("henry"))]),
            Record::new(vec![Value::Int(9), Value::VarChar(String::from("irene"))]),
            Record::new(vec![Value::Int(10), Value::VarChar(String::from("judy"))]),
        ],
    );

    let todo_table = Table::new(
        String::from("todo"),
        vec![
            (String::from("id"), DataType::Int),
            (String::from("user_id"), DataType::Int),
            (String::from("title"), DataType::VarChar(10)),
        ],
        vec![
            Record::new(vec![
                Value::Int(1),
                Value::Int(1),
                Value::VarChar(String::from("alice 1")),
            ]),
            Record::new(vec![
                Value::Int(2),
                Value::Int(1),
                Value::VarChar(String::from("alice 2")),
            ]),
            Record::new(vec![
                Value::Int(3),
                Value::Int(2),
                Value::VarChar(String::from("bob 1")),
            ]),
            Record::new(vec![
                Value::Int(4),
                Value::Int(2),
                Value::VarChar(String::from("bob 2")),
            ]),
            Record::new(vec![
                Value::Int(5),
                Value::Int(3),
                Value::VarChar(String::from("charlie 1")),
            ]),
            Record::new(vec![
                Value::Int(6),
                Value::Int(3),
                Value::VarChar(String::from("charlie 2")),
            ]),
            Record::new(vec![
                Value::Int(7),
                Value::Int(4),
                Value::VarChar(String::from("david 1")),
            ]),
            Record::new(vec![
                Value::Int(8),
                Value::Int(4),
                Value::VarChar(String::from("david 2")),
            ]),
            Record::new(vec![
                Value::Int(9),
                Value::Int(5),
                Value::VarChar(String::from("eve 1")),
            ]),
            Record::new(vec![
                Value::Int(10),
                Value::Int(5),
                Value::VarChar(String::from("eve 2")),
            ]),
            Record::new(vec![
                Value::Int(11),
                Value::Int(6),
                Value::VarChar(String::from("frank 1")),
            ]),
            Record::new(vec![
                Value::Int(12),
                Value::Int(6),
                Value::VarChar(String::from("frank 2")),
            ]),
            Record::new(vec![
                Value::Int(13),
                Value::Int(7),
                Value::VarChar(String::from("grace 1")),
            ]),
            Record::new(vec![
                Value::Int(14),
                Value::Int(7),
                Value::VarChar(String::from("grace 2")),
            ]),
            Record::new(vec![
                Value::Int(15),
                Value::Int(8),
                Value::VarChar(String::from("henry 1")),
            ]),
        ],
    );
    executer.storage.flush(&user_table);
    executer.storage.flush(&todo_table);
}
