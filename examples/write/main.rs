use ubdb::{
    storage::write,
    table::{Column, ColumnData, ColumnType, Table},
};

fn main() {
    let user_table = Table::new(
        "user".to_string(),
        Column {
            ty: ColumnType::Int,
            key: "id".to_string(),
        },
        vec![
            Column {
                ty: ColumnType::Varchar(190),
                key: "name".to_string(),
            },
            Column {
                ty: ColumnType::Varchar(190),
                key: "email".to_string(),
            },
        ],
    );

    let data = vec![
        ColumnData::from_int(1),
        ColumnData::from_string("John".to_string()),
        ColumnData::from_string("example@example.com".to_string()),
    ];

    write(user_table, data);
}
