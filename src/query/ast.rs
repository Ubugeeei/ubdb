#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    // (is_all, columns)
    Select(bool, Vec<String>),

    // (key_name, value)
    Set(Vec<(String, i32)>),

    // (table_name, (column_name, data_type)[])
    CreateTable(String, Vec<(String, DataType)>),

    Exit,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    VarChar(u16),
}
