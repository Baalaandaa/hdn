use serde::{Deserialize, Serialize};

/// Response to Store request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "response_status")]
#[serde(rename_all = "lowercase")]
pub enum StoreResponse {
    /// Means that (key, value) was successfully added to storage
    Success {},
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "response_status")]
#[serde(rename_all = "lowercase")]
pub enum LoadResponse {
    /// Means that key was successfully found in storage and returns requested hash
    Success {
        requested_key: String,
        requested_hash: String,
    },

    /// Key was not found in storage
    #[serde(rename = "key not found")]
    KeyNotFound {},
}

#[cfg(test)]
mod test {
    use crate::models::response::{LoadResponse, StoreResponse};

    #[test]
    pub fn load_deserialize_test() {
        let str: String = serde_json::to_string(&LoadResponse::Success {
            requested_key: "a".parse().unwrap(),
            requested_hash: "b".parse().unwrap(),
        })
        .unwrap();
        let eq = r#"{"response_status":"success","requested_key":"a","requested_hash":"b"}"#;
        assert_eq!(str, eq);

        let str: String = serde_json::to_string(&LoadResponse::KeyNotFound {}).unwrap();
        let eq = r#"{"response_status":"key not found"}"#;
        assert_eq!(str, eq)
    }

    #[test]
    pub fn store_deserialize_test() {
        let str: String = serde_json::to_string(&StoreResponse::Success {}).unwrap();
        let eq = r#"{"response_status":"success"}"#;
        assert_eq!(str, eq)
    }
}
