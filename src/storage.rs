use crate::table::{ColumnData, ColumnType, Table};

const STORAGE_FILE_PATH: &str = "db.ubdb";

pub fn write(table: Table, data: Vec<ColumnData>) {
    let mut buffer = Vec::new();

    // table def
    buffer.extend_from_slice(&table.as_bytes());

    // data
    for (idx, column_data) in data.iter().enumerate() {
        let ty = if idx == 0 {
            &table.pk.ty
        } else {
            &table.columns[idx - 1].ty
        };

        match ty {
            ColumnType::Int => {
                let value = &column_data.value;
                let pad = 4 - value.len();
                let pad = vec![0; pad];
                buffer.extend_from_slice(&pad);
                buffer.extend_from_slice(value);
            }
            ColumnType::Varchar(len) => {
                let value = &column_data.value;
                let pad = len - value.len() as u32;
                let pad = vec![0; pad as usize];
                buffer.extend_from_slice(&pad);
                buffer.extend_from_slice(value);
            }
        }
    }

    std::fs::write(STORAGE_FILE_PATH, buffer).unwrap();
}

pub fn read() -> (Table, Vec<ColumnData>) {
    let buffer = std::fs::read(STORAGE_FILE_PATH).unwrap();
    
    let mut offset = 0;

    let table = Table::from_bytes(&buffer);
    offset += table.as_bytes().len();

    let mut values = Vec::new();

    // pk
    let ty = &table.pk.ty;
    let value = match ty {
        ColumnType::Int => {
            let value = &buffer[offset..offset + 4];
            offset += 4;
            let value = value.iter().enumerate().fold(0i32, |acc, (idx, byte)| {
                let shift = (3 - idx) * 8;
                acc | ((*byte as i32) << shift)
            });
            ColumnData::from_int(value)
        }
        ColumnType::Varchar(len) => {
            let value = &buffer[offset..offset + *len as usize];
            offset += *len as usize;
            let value = String::from_utf8(value.to_vec()).unwrap();
            ColumnData::from_string(value)
        }
    };
    values.push(value);

    // columns
    for column in table.columns.iter() {
        let ty = &column.ty;
        let value = match ty {
            ColumnType::Int => {
                let value = &buffer[offset..offset + 4];
                offset += 4;
                let value = String::from_utf8(value.to_vec()).unwrap();
                ColumnData::from_int(value.parse::<i32>().unwrap())
            }
            ColumnType::Varchar(len) => {
                let value = &buffer[offset..offset + *len as usize];
                offset += *len as usize;
                let value = String::from_utf8(value.to_vec()).unwrap();
                ColumnData::from_string(value)
            }
        };
        values.push(value);
    }

    (table, values)
}
