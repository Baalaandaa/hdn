use serde::{Deserialize, Serialize};

/// Struct for entry message
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryMessage {
    pub student_name: String,
}

#[cfg(test)]
mod test {
    use crate::models::entry::EntryMessage;

    #[test]
    pub fn serialize_works() {
        let req = r#"{"student_name":"Kirill🤡"}"#;
        let serialized_request: EntryMessage = serde_json::from_str(req).unwrap();
        assert_eq!(serialized_request.student_name, "Kirill🤡");
        let deserialized_request: String = serde_json::to_string(&serialized_request).unwrap();
        assert_eq!(req, deserialized_request);
    }

    #[test]
    pub fn invalid_json() {
        let req = r#"{"student_name":1}"#;
        let serialized_request: Result<EntryMessage, _> = serde_json::from_str(req);
        assert!(serialized_request.is_err());

        let req = r#"{"student_name":"}"#;
        let serialized_request: Result<EntryMessage, _> = serde_json::from_str(req);
        assert!(serialized_request.is_err());
    }
}
