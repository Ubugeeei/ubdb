use crate::core::table::{DataType, Table, Value};

use super::{DataTypeByteMap, Storage};

impl Storage {
    pub fn flush(&self, table: &Table) {
        let path = self.get_table_storage_path(&table.name);
        let bytes = Self::table_to_bytes(table);

        let dir = std::path::Path::new(&path).parent().unwrap();
        std::fs::create_dir_all(dir).unwrap();

        std::fs::write(path, bytes).unwrap();
    }

    fn table_to_bytes(table: &Table) -> Vec<u8> {
        let mut b = vec![];

        // name
        b.extend_from_slice(&(table.name.len() as u8).to_be_bytes());
        b.extend_from_slice(table.name.as_bytes());

        // columns
        b.extend_from_slice(&(table.columns.len() as u16).to_be_bytes());
        for (column_name, data_type) in table.columns.iter() {
            b.extend_from_slice(&(column_name.len() as u16).to_be_bytes());
            b.extend_from_slice(column_name.as_bytes());
            b.extend_from_slice(&Self::data_type_to_bytes(data_type));
        }

        // records
        b.extend_from_slice(&(table.records.len() as u16).to_be_bytes());
        for record in table.records.iter() {
            for (idx, value) in record.values.iter().enumerate() {
                b.extend_from_slice(&Self::test_value_as_bytes(value, &table.columns[idx].1));
            }
        }
        b
    }

    fn data_type_to_bytes(data_type: &DataType) -> Vec<u8> {
        match data_type {
            DataType::Int => vec![DataTypeByteMap::INT],
            DataType::VarChar(size) => {
                let mut b = vec![DataTypeByteMap::VARCHAR];
                b.extend_from_slice(&size.to_be_bytes());
                b
            }
        }
    }

    fn test_value_as_bytes(value: &Value, data_type: &DataType) -> Vec<u8> {
        match value {
            Value::Int(value) => value.to_be_bytes().to_vec(),
            Value::VarChar(value) => match data_type {
                DataType::VarChar(size) => {
                    let mut b = vec![];
                    b.extend_from_slice(&(value.len() as u16).to_be_bytes());
                    b.extend_from_slice(value.as_bytes());
                    b
                }
                _ => panic!("data type mismatch"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::table::Record;

    use super::*;

    #[test]
    fn test_data_type_as_bytes() {
        assert_eq!(
            Storage::data_type_to_bytes(&DataType::Int),
            vec![DataTypeByteMap::INT]
        );
        assert_eq!(
            Storage::data_type_to_bytes(&DataType::VarChar(0)),
            vec![DataTypeByteMap::VARCHAR, 0x00, 0x00]
        );
        assert_eq!(
            Storage::data_type_to_bytes(&DataType::VarChar(10)),
            vec![DataTypeByteMap::VARCHAR, 0x00, 0x0a]
        );
        assert_eq!(
            Storage::data_type_to_bytes(&DataType::VarChar(0xffff)),
            vec![DataTypeByteMap::VARCHAR, 0xff, 0xff]
        );
    }

    #[test]
    fn test_value_as_bytes() {
        assert_eq!(
            Storage::test_value_as_bytes(&Value::Int(1), &DataType::Int),
            vec![0x00, 0x00, 0x00, 0x01]
        );

        assert_eq!(
            Storage::test_value_as_bytes(
                &Value::VarChar(String::from("hello")),
                &DataType::VarChar(10)
            ),
            vec![0, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]
        );
    }

    #[test]
    fn test_table_as_bytes() {
        let mut user_table = Table::new(
            String::from("user"),
            vec![
                (String::from("id"), DataType::Int),
                (String::from("name"), DataType::VarChar(10)),
            ],
            vec![],
        );

        user_table.insert(Record::new(vec![
            Value::Int(1),
            Value::VarChar(String::from("alice")),
        ]));

        user_table.insert(Record::new(vec![
            Value::Int(2),
            Value::VarChar(String::from("bob")),
        ]));

        assert_eq!(
            Storage::table_to_bytes(&user_table),
            vec![
                0x04, // name length
                0x75, 0x73, 0x65, 0x72, // user
                0x00, 0x02, // 2 columns
                0x00, 0x02, // column name length
                0x69, 0x64, // id
                0x00, // int
                0x00, 0x04, // column name length
                0x6e, 0x61, 0x6d, 0x65, // name
                0x0a, 0x00, 0x0a, // varchar(10)
                0x00, 0x02, // records length
                0x00, 0x00, 0x00, 0x01, // 1
                0x00, 0x05, // alice length
                0x61, 0x6c, 0x69, 0x63, 0x65, // alice
                0x00, 0x00, 0x00, 0x02, // 2
                0x00, 0x03, // bob length
                0x62, 0x6f, 0x62, // bob
            ]
        )
    }
}
