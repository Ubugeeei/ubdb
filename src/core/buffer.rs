use std::collections::BTreeMap;

pub struct BufferPool {
    pub body: BTreeMap<String, i32>,
}

impl BufferPool {
    pub fn new() -> Self {
        Self {
            body: BTreeMap::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = Self::new();
        let mut key = String::new();
        let mut value = String::new();
        let mut is_key = true;
        for byte in bytes.iter() {
            if *byte == b'=' {
                is_key = false;
            } else if *byte == b'\n' {
                buffer.body.insert(key, value.parse::<i32>().unwrap());
                key = String::new();
                value = String::new();
                is_key = true;
            } else if is_key {
                key.push(*byte as char);
            } else {
                value.push(*byte as char);
            }
        }
        buffer
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (key, value) in self.body.iter() {
            bytes.extend_from_slice(key.as_bytes());
            bytes.push(b'=');
            bytes.extend_from_slice(value.to_string().as_bytes());
            bytes.push(b'\n');
        }
        bytes
    }
}
