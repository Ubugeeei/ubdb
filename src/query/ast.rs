#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    // (is_all, columns)
    Select(bool, Vec<String>),

    // (table_name, (key_name, value))
    Update(String, Vec<(String, Value)>),

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
