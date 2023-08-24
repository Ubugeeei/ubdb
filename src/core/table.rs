#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<(String, DataType)>,
    pub records: Vec<Record>,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    VarChar(usize),
}

#[derive(Debug, PartialEq)]
pub struct Record {
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    VarChar(String),
}
