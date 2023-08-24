mod buffer;
pub mod storage;
pub mod table;

use super::query::ast::{DataType, QueryStatement, Value};

use self::{buffer::BufferPool, storage::Storage, table::Table};

pub struct Executer {
    buffer: BufferPool,
    pub storage: Storage,
}

impl Executer {
    pub fn new(storage_path: String) -> Self {
        let storage = Storage::new(storage_path);
        let buffer = BufferPool::new();
        Self { buffer, storage }
    }

    /// return value means whether to continue the repl
    /// if return false, then exits
    pub fn execute(&mut self, query: Vec<QueryStatement>) -> bool {
        for stmt in query.iter() {
            match stmt {
                QueryStatement::CreateTable(table_name, columns) => {
                    self.create_table(table_name.clone(), columns.clone())
                }
                QueryStatement::Select(table_name, is_all, column, cond) => {
                    self.select(table_name.clone(), *is_all, column.clone(), cond.clone())
                }
                QueryStatement::Update(table_name, set, cond) => {
                    self.update(table_name.clone(), set.clone(), cond.clone())
                }
                QueryStatement::Exit => {
                    println!("bye!");
                    return false;
                }
            }
        }
        true
    }

    fn create_table(&mut self, table_name: String, columns: Vec<(String, DataType)>) {
        let columns = columns
            .iter()
            .map(|(name, data_type)| {
                (
                    name.clone(),
                    match data_type {
                        DataType::Int => table::DataType::Int,
                        DataType::VarChar(size) => table::DataType::VarChar(*size),
                    },
                )
            })
            .collect();
        let table = Table::new(table_name, columns, vec![]);
        self.storage.flush(&table);
        self.buffer.body.push(table);
    }

    fn select(
        &mut self,
        table_name: String,
        is_all: bool,
        columns: Vec<String>,
        cond: Option<(String, Value)>,
    ) {
        let binding = self
            .storage
            .load(&table_name)
            .expect("table should be in storage"); // TODO: error handling

        let table = self
            .buffer
            .body
            .iter()
            .find(|table| table.name == table_name)
            .unwrap_or(&binding); // TODO: push to buffer

        let mut rows = table.rows.clone();

        // filter by where
        if let Some((key_name, value)) = cond {
            let key_idx = table
                .columns
                .iter()
                .position(|(name, _)| name == &key_name)
                .expect("key_name should be in columns");

            rows.retain(|row| {
                let _value = &row.values[key_idx];
                match &value {
                    Value::Int(v) => {
                        if let table::Value::Int(cond_value) = _value {
                            v == cond_value
                        } else {
                            false
                        }
                    }
                    Value::VarChar(v) => {
                        if let table::Value::VarChar(cond_value) = _value {
                            v == cond_value
                        } else {
                            false
                        }
                    }
                }
            });
        }

        if !is_all {
            rows = rows
                .into_iter()
                .map(|row| {
                    let mut new_row = row;
                    let target_indexes = table
                        .columns
                        .iter()
                        .enumerate()
                        .filter(|(_, (name, _))| columns.contains(name))
                        .map(|(idx, _)| idx)
                        .collect::<Vec<_>>();
                    new_row.values = new_row
                        .values
                        .into_iter()
                        .enumerate()
                        .filter(|(idx, _)| target_indexes.contains(idx))
                        .map(|(_, value)| value)
                        .collect();
                    new_row
                })
                .collect();
        }

        // print
        if rows.is_empty() {
            println!("Empty set");
        } else {
            let mut column_names = table
                .columns
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();
            if !is_all {
                column_names.retain(|name| columns.contains(name));
            }
            let column_names = column_names.join(", ");
            println!("{}", column_names);
            for row in rows {
                let values = row
                    .values
                    .into_iter()
                    .map(|value| match value {
                        table::Value::Int(v) => v.to_string(),
                        table::Value::VarChar(v) => v,
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}", values);
            }
        }
    }

    fn update(&mut self, table_name: String, set: Vec<(String, Value)>, cond: (String, Value)) {
        let mut binding = self
            .storage
            .load(&table_name)
            .expect("table should be in storage"); // TODO: error handling

        let table = self
            .buffer
            .body
            .iter_mut()
            .find(|table| table.name == table_name)
            .unwrap_or(&mut binding);

        let rows = table.rows.clone();
        let mut new_rows = Vec::new();
        for row in rows {
            let mut new_row = row.clone();
            let key_idx = table
                .columns
                .iter()
                .position(|(name, _)| name == &cond.0)
                .expect("key_name should be in columns");
            let value = &row.values[key_idx];

            match cond.1 {
                Value::Int(cond_value) => {
                    if let table::Value::Int(value) = value {
                        if value == &cond_value {
                            for (name, value) in set.iter() {
                                let idx = table
                                    .columns
                                    .iter()
                                    .position(|(column_name, _)| column_name == name)
                                    .expect("key_name should be in columns");
                                if let Value::Int(v) = value {
                                    new_row.values[idx] = table::Value::Int(*v);
                                }
                            }
                        }
                    }
                }
                Value::VarChar(ref cond_value) => {
                    if let table::Value::VarChar(value) = value {
                        if value == cond_value {
                            for (name, value) in set.iter() {
                                let idx = table
                                    .columns
                                    .iter()
                                    .position(|(column_name, _)| column_name == name)
                                    .expect("key_name should be in columns");
                                if let Value::VarChar(v) = value {
                                    new_row.values[idx] = table::Value::VarChar(v.clone());
                                }
                            }
                        }
                    }
                }
            };

            new_rows.push(new_row);
        }
        table.rows = new_rows;

        // sync
        self.storage.flush(table);
    }
}
