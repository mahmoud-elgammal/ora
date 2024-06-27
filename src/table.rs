use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
            columns: HashMap::new(),
            select_columns: Vec::new(),
        }
    }
}