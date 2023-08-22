#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    Select { is_all: bool, columns: Vec<String> },

    // (key_name, value)
    Set(Vec<(String, i32)>),

    Exit,
}
