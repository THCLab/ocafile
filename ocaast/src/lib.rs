use serde::{Serialize, Serializer};
use strum_macros::Display;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub struct OCAAst {
    pub version: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub object_kind: ObjectKind,
    pub content: Option<ObjectContent>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CommandType {
    Add,
    Remove,
    Modify,
    From,
}

#[derive(Debug, PartialEq)]
pub enum ObjectKind {
    CaptureBase,
    Overlay(OverlayType),
}
#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObjectContent {
    CaptureBase(CaptureBaseContent),
    Overlay(OverlayContent),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum AttributeType {
    Boolean,
    #[serde(rename = "Array[Boolean]")]
    ArrayBoolean,
    Binary,
    #[serde(rename = "Array[Binary]")]
    ArrayBinary,
    Text,
    #[serde(rename = "Array[Text]")]
    ArrayText,
    Numeric,
    #[serde(rename = "Array[Numeric]")]
    ArrayNumeric,
    DateTime,
    #[serde(rename = "Array[DateTime]")]
    ArrayDateTime,
    Reference,
    #[serde(rename = "Array[Reference]")]
    ArrayReference,
}

#[derive(Debug, PartialEq, Serialize, Display)]
pub enum OverlayType {
    Label,
    Information,
    Encoding,
    CharacterEncoding,
    Format,
    Meta,
    Standard,
    Cardinality,
    Conditional,
    Conformance,
    EntryCode,
    Entry,
    Unit,
    AttributeMapping,
    EntryCodeMapping,
    Subset,
    UnitMapping,
    Layout,
    Sensitivity,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CaptureBaseContent {
    pub attributes: Option<HashMap<String, NestedValue>>,
    pub properties: Option<HashMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct OverlayContent {
    pub capture_base_id: Option<String>, // TODO do we need it in AST?
    pub properties: Option<HashMap<String, NestedValue>>,
    pub body: Option<HashMap<String, NestedValue>>, // maybe we should have body and attributes
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NestedValue {
    Value(String),
    Object(HashMap<String, NestedValue>),
    Reference(String),
    Array(Vec<NestedValue>),
}

pub(crate) trait Content {}

impl Content for CaptureBaseContent {}
impl Content for OverlayContent {}

impl OCAAst {
    pub fn new() -> Self {
        OCAAst {
            version: String::from("1.0"),
            commands: Vec::new(),
        }
    }
}

impl Serialize for ObjectKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ObjectKind::CaptureBase => serializer.serialize_str("CaptureBase"),
            ObjectKind::Overlay(overlay_type) => {
                serializer.serialize_str(overlay_type.to_string().as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocaast_serialize() {
        let mut attributes = HashMap::new();
        let mut properties = HashMap::new();
        let mut person = HashMap::new();
        person.insert("name".to_string(), NestedValue::Value("Text".to_string()));

        attributes.insert("person".to_string(), NestedValue::Object(person));
        attributes.insert("test".to_string(), NestedValue::Value("test".to_string()));
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(ObjectContent::CaptureBase(CaptureBaseContent {
                attributes: Some(attributes),
                properties: Some(properties),
            })),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        let serialized = serde_json::to_string(&ocaast).unwrap();
        assert_eq!(
            serialized,
            r#"{"version":"1.0","commands":[{"type":"Add","object_kind":"CaptureBase","content":{"attributes":{"test":"test","person":{"name":"Text"}},"properties":{"test":"test"}}}]}"#
        );
    }
}
