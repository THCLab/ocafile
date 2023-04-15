use serde::{
    ser::{SerializeStruct, SerializeMap},
    Serialize,
};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct OCAAst {
    pub(crate) version: String,
    pub(crate) commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Command {
    #[serde(rename = "type")]
    pub(crate) kind: CommandType,
    pub(crate) object_kind: ObjectKind,
    pub(crate) content: Option<ObjectContent>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum CommandType {
    Add,
    Remove,
    Modify,
    From,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum ObjectKind {
    CaptureBase,
    Overlay(OverlayType),
}
#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum ObjectContent {
    CaptureBase(CaptureBaseContent),
    Overlay(OverlayContent),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum AttributeType {
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

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum OverlayType {
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
pub(crate) struct CaptureBaseContent {
    pub(crate) attributes: Option<HashMap<String, NestedValue>>,
    pub(crate) properties: Option<HashMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct OverlayContent {
    pub(crate) capture_base_id: String,
    pub(crate) properties: HashMap<String, NestedValue>,
    pub(crate) body: HashMap<String, NestedValue>, // maybe we should have body and attributes
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum NestedValue {
    Value(String),
    Object(HashMap<String, NestedValue>),
    Reference(String),
    Array(Vec<NestedValue>),
}

pub(crate) trait Content {}

impl Content for CaptureBaseContent {}
impl Content for OverlayContent {}

impl OCAAst {
    pub(crate) fn new() -> Self {
        OCAAst {
            version:  String::from("1.0"),
            commands: Vec::new(),
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
