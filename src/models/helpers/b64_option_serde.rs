//! Custom (de)serialization functions for optional base64-encoded byte arrays.
//!
//! This module provides custom serialization and deserialization functions for
//! handling `Option<Vec<u8>>` types that are base64-encoded.

use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Deserializer, Serializer};

/// Serializes an optional `Vec<u8>` as a base64-encoded string.
///
/// If the input is `Some(Vec<u8>)`, it will be base64-encoded and serialized as a string.
/// If the input is `None`, it will be serialized as a JSON `null`.
/// If the input is invalid base64, an error will be returned.
pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bytes {
        Some(bytes) => serializer.serialize_str(general_purpose::STANDARD.encode(bytes).as_str()),
        None => serializer.serialize_none(),
    }
}

/// Deserializes a base64-encoded string into an optional `Vec<u8>`.
///
/// If the input is a JSON `null`, it will be deserialized as `None`.
/// If the input is a base64-encoded string, it will be deserialized into a `Some(Vec<u8>)`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes_option: Option<String> = Option::deserialize(deserializer)?;

    match bytes_option {
        Some(bytes) => {
            let deserialized_bytes = general_purpose::STANDARD
                .decode(bytes)
                .map_err(serde::de::Error::custom)?;
            Ok(Some(deserialized_bytes))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        #[serde(with = "super")]
        pub content: Option<Vec<u8>>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        pub descriptors: Option<Vec<TestStruct>>,
    }

    #[test]
    fn test_serialize_base64_opt() {
        let data = TestStruct {
            content: Some(vec![104, 101, 108, 108, 111]),
        };
        let result = serde_json::to_value(&data).expect("Failed to serialize bytes");
        assert_eq!(result, json!({"content": "aGVsbG8="}));
    }

    #[test]
    fn test_serialize_base64_opt_none() {
        let data = TestStruct { content: None };
        let result = serde_json::to_value(&data).expect("Failed to serialize bytes");
        assert_eq!(result, json!({ "content": null }));
    }

    #[test]
    fn test_deserialize_base64_opt() {
        let value = json!({"content": "aGVsbG8="});
        let data: TestStruct = serde_json::from_value(value).expect("Failed to deserialize bytes");
        assert_eq!(
            data,
            TestStruct {
                content: Some(vec![104, 101, 108, 108, 111])
            }
        );
    }

    #[test]
    fn test_deserialize_base64_opt_none() {
        let value = json!({ "content": null });
        let data: TestStruct = serde_json::from_value(value).expect("Failed to deserialize bytes");
        assert_eq!(data, TestStruct { content: None });
    }

    #[test]
    fn test_nested_serialize_base64_opt() {
        let data = TestData {
            descriptors: Some(vec![
                TestStruct {
                    content: Some(vec![104, 101, 108, 108, 111]),
                },
                TestStruct { content: None },
            ]),
        };
        let result = serde_json::to_value(&data).expect("Failed to serialize bytes");
        assert_eq!(
            result,
            json!({"descriptors": [{"content": "aGVsbG8="}, {"content": null}]})
        );
    }

    #[test]
    fn test_nested_deserialize_base64_opt() {
        let value = json!({"descriptors": [{"content": "aGVsbG8="}, {"content": null}]});
        let data: TestData = serde_json::from_value(value).expect("Failed to deserialize bytes");
        assert_eq!(
            data,
            TestData {
                descriptors: Some(vec![
                    TestStruct {
                        content: Some(vec![104, 101, 108, 108, 111]),
                    },
                    TestStruct { content: None },
                ])
            }
        );
    }
}
