use crate::Error;

/// Validate an incoming request schema.
#[allow(dead_code)]
pub fn validate_request(body: &str) -> core::result::Result<(), Error> {
    let parsed: core::result::Result<serde_json::Value, _> = serde_json::from_str(body);
    match parsed {
        Ok(val) => {
            if val.get("model").is_none() {
                return Err(Error::Validation(
                    alloc::string::String::from("missing required field: model"),
                ));
            }
            if val.get("messages").is_none() {
                return Err(Error::Validation(
                    alloc::string::String::from("missing required field: messages"),
                ));
            }
            Ok(())
        }
        Err(e) => Err(Error::Json(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_request() {
        let body = r#"{"model":"gpt-4","messages":[{"role":"user","content":"hi"}]}"#;
        assert!(validate_request(body).is_ok());
    }

    #[test]
    fn test_missing_model() {
        let body = r#"{"messages":[{"role":"user","content":"hi"}]}"#;
        assert!(validate_request(body).is_err());
    }
}
