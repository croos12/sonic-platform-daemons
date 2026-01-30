use std::collections::HashMap;
use std::sync::RwLock;

pub const STATE_DB: &str = "";
pub const CHASSIS_STATE_DB: &str = "";

pub type FieldValuePairs = Vec<(String, String)>;

pub struct Table {
    table_name: String,
    mock_dict: RwLock<HashMap<String, HashMap<String, String>>>,
}

impl Table {
    pub fn new(_db: &str, table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            mock_dict: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.table_name
    }

    pub fn del(&self, key: &str) {
        self.mock_dict.write().unwrap().remove(key);
    }

    pub fn set(&self, key: &str, fvs: &FieldValuePairs) {
        let fv_map: HashMap<String, String> = fvs.iter().cloned().collect();
        self.mock_dict.write().unwrap().insert(key.to_string(), fv_map);
    }

    pub fn get(&self, key: &str) -> Option<HashMap<String, String>> {
        self.mock_dict.read().unwrap().get(key).cloned()
    }

    pub fn get_size(&self) -> usize {
        self.mock_dict.read().unwrap().len()
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.mock_dict.read().unwrap().keys().cloned().collect()
    }

    pub fn clear(&self) {
        self.mock_dict.write().unwrap().clear();
    }
}

pub fn field_value_pairs(fvs: Vec<(&str, &str)>) -> FieldValuePairs {
    fvs.into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
