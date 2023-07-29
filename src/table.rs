#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub pk: Column,
    pub columns: Vec<Column>,
    // pub relation: Vec<TableRelation>,
}

impl Table {
    pub fn new(name: String, pk: Column, columns: Vec<Column>) -> Self {
        Table { name, pk, columns }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        let name_bytes = self.name.as_bytes();
        buffer.push(name_bytes.len() as u8);
        buffer.extend_from_slice(name_bytes);

        buffer.extend_from_slice(&self.pk.as_bytes());

        buffer.push(self.columns.len() as u8);
        for column in self.columns.iter() {
            buffer.extend_from_slice(&column.as_bytes());
        }

        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;

        let name_len = bytes[offset] as usize;
        offset += 1;
        let name = String::from_utf8(bytes[offset..offset + name_len].to_vec()).unwrap();
        offset += name_len;

        let pk = Column::from_bytes(&bytes[offset..]);
        offset += pk.as_bytes().len();

        let columns_len = bytes[offset] as usize;
        offset += 1;
        let mut columns = Vec::new();
        for _ in 0..columns_len {
            let column = Column::from_bytes(&bytes[offset..]);
            offset += column.as_bytes().len();
            columns.push(column);
        }

        Table { name, pk, columns }
    }
}

#[derive(Debug)]
pub enum ColumnType {
    Int,
    Varchar(u32),
}

impl ColumnType {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            ColumnType::Int => vec![0],
            ColumnType::Varchar(len) => {
                let len = len.to_be_bytes();
                let mut buffer = vec![1];
                buffer.extend_from_slice(&len);
                buffer
            }
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            ColumnType::Int => 4,
            ColumnType::Varchar(len) => *len,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => ColumnType::Int,
            1 => {
                let mut len_bytes = [0; 4];
                len_bytes.copy_from_slice(&bytes[1..5]);
                let len = u32::from_be_bytes(len_bytes);
                ColumnType::Varchar(len)
            }
            _ => panic!("Invalid column type"),
        }
    }
}

#[derive(Debug)]
pub struct Column {
    pub ty: ColumnType,
    pub key: String,
}

impl Column {
    pub fn new(ty: ColumnType, key: String) -> Self {
        Column { ty, key }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&self.ty.as_bytes());

        let key_bytes = self.key.as_bytes();
        buffer.push(key_bytes.len() as u8);
        buffer.extend_from_slice(key_bytes);

        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let ty = ColumnType::from_bytes(&bytes[..5]);

        let mut offset = match ty {
            ColumnType::Int => 1,
            ColumnType::Varchar(_) => 5,
        };

        let key_len = bytes[offset] as usize;
        offset += 1;
        let key = String::from_utf8(bytes[offset..offset + key_len].to_vec()).unwrap();
        // offset += key_len;

        Column { ty, key }
    }
}

// #[derive(Debug)]
// pub struct TableRelation {
//     pub target_name: String,
//     pub relation_type: TableRelationType,
// }

// #[derive(Debug)]
// pub enum TableRelationType {
//     OneToOne,
//     OneToMany,
//     ManyToOne,
//     ManyToMany,
// }

#[derive(Debug)]
pub struct ColumnData {
    pub value: Vec<u8>,
}

impl ColumnData {
    pub fn from_string(value: String) -> Self {
        ColumnData {
            value: value.as_bytes().to_vec(),
        }
    }

    pub fn from_int(value: i32) -> Self {
        ColumnData {
            value: value.to_be_bytes().to_vec(),
        }
    }

    pub fn into_display(&self, column: &Column) -> String {
        match column.ty {
            ColumnType::Int => {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&self.value);
                let value = i32::from_be_bytes(bytes);
                value.to_string()
            }
            ColumnType::Varchar(_) => String::from_utf8(self.value.clone()).unwrap(),
        }
    }
}
