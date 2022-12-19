use serde::{Deserialize, Serialize};

/// Request from client
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "request_type")]
#[serde(rename_all = "lowercase")]
pub enum Request {
    /// Store request
    /// Adds (key, hash) pair into KV storage
    Store { key: String, hash: String },

    /// Load request
    /// Gets value by key from KV storage
    Load { key: String },
}

#[cfg(test)]
mod test {
    use crate::models::request::Request;

    #[test]
    pub fn store_serialize_deserialize_works() {
        let req = r#"{"request_type":"store","key":"winner","hash":"argentina🇦🇷🤡"}"#;
        let serialized_request: Request = serde_json::from_str(req).unwrap();
        let deserialized_request: String = serde_json::to_string(&serialized_request).unwrap();
        assert_eq!(req, deserialized_request);

        let req = r#"{"request_type":"store","key":"","hash":""}"#;
        let serialized_request: Request = serde_json::from_str(req).unwrap();
        let deserialized_request: String = serde_json::to_string(&serialized_request).unwrap();
        assert_eq!(req, deserialized_request);
    }

    #[test]
    pub fn invalid_json() {
        let req = r#"{"request_type":"store","key":1,"hash":""}"#;
        let serialized_request: Result<Request, _> = serde_json::from_str(req);
        assert!(serialized_request.is_err());

        let req = r#"{"request_type":"load","key":1"}"#;
        let serialized_request: Result<Request, _> = serde_json::from_str(req);
        assert!(serialized_request.is_err());

        let req = r#"{"request_type":"' DROP DATABASE `users`;","key":1"}"#;
        let serialized_request: Result<Request, _> = serde_json::from_str(req);
        assert!(serialized_request.is_err());
    }
}
