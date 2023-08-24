#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    // TODO: AND, OR, others
    // (table_name, is_all, columns, where(key_name, value))
    Select(String, bool, Vec<String>, Option<(String, Value)>),

    // TODO: AND, OR, others
    // (table_name, set(key_name, value)[], where(key_name, value))
    Update(String, Vec<(String, Value)>, (String, Value)),

    // (table_name, (column_name, data_type)[])
    CreateTable(String, Vec<(String, DataType)>),

    Exit,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    VarChar(u16),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    VarChar(String),
}
