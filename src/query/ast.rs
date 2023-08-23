#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    // (is_all, columns)
    Select(bool, Vec<String>),

    // (key_name, value)
    Set(Vec<(String, i32)>),

    Exit,
}
