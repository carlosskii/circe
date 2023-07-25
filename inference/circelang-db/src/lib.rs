use std::collections::HashMap;


pub struct Database {
    entries: Vec<(String, String)>,
    lookup: HashMap<String, Vec<u64>>
}

impl Database {
    pub fn new() -> Database {
        Database {
            entries: Vec::new(),
            lookup: HashMap::new()
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
