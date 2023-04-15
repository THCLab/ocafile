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

#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Add(ObjectKind, ObjectContent),
    Remove(ObjectKind, ObjectContent),
    Modify(ObjectKind, ObjectContent),
    From(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum ObjectKind {
    CaptureBase,
    Overlay(OverlayType),
}
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub(crate) struct CaptureBaseContent {
    pub(crate) attributes: Option<HashMap<String, NestedValue>>,
    pub(crate) properties: Option<HashMap<String, NestedValue>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct OverlayContent {
    pub(crate) capture_base_id: String,
    pub(crate) properties: HashMap<String, NestedValue>,
    pub(crate) body: HashMap<String, NestedValue>,
}

#[derive(Debug, PartialEq)]
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

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Command::Add(kind, content) => {
                let mut state = serializer.serialize_struct("Command", 3)?;
                state.serialize_field("type", "ADD")?;
                state.serialize_field("objectKind", kind)?;
                match content {
                    ObjectContent::CaptureBase(content) => {
                        match &content.attributes {
                            Some(nested_object) => {
                                state.serialize_field("attributes", &nested_object)?;
                            }
                            None => {}
                        }
                        match &content.properties {
                            Some(nested_object) => {

                                state.serialize_field("properties", &nested_object)?;
                            }
                            None => {}
                        }
                    }
                    ObjectContent::Overlay(content) => {
                        state.serialize_field("captureBaseId", &content.capture_base_id)?;
                    }
                }
                state.end()
            }
            Command::Remove(kind, content) => {
                let mut state = serializer.serialize_struct("Command", 3)?;
                state.serialize_field("type", "REMOVE")?;
                state.serialize_field("objectKind", kind)?;
               // state.serialize_field("properties", content)?;
                state.end()
            }
            Command::Modify(kind, content) => {
                let mut state = serializer.serialize_struct("Command", 3)?;
                state.serialize_field("type", "MODIFY")?;
                state.serialize_field("objectKind", kind)?;
              //  state.serialize_field("properties", content)?;
                state.end()
            }
            Command::From(path) => {
                let mut state = serializer.serialize_struct("Command", 2)?;
                state.serialize_field("type", "FROM")?;
                state.serialize_field("path", path)?;
                state.end()
            }
        }
    }
}


impl Serialize for NestedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NestedValue::Value(value) => serializer.serialize_str(value),
            NestedValue::Object(object) => {
                let mut state = serializer.serialize_map(Some(object.len()))?;
                for (k, v) in object {
                    state.serialize_entry(k, v)?;
                }
                state.end()
            }
            NestedValue::Reference(reference) => {
                let mut state = serializer.serialize_struct("NestedObject", 1)?;
                state.serialize_field("type", "Reference")?;
                state.serialize_field("value", reference)?;
                state.end()
            }
            NestedValue::Array(array) => {
                let mut state = serializer.serialize_struct("NestedObject", 1)?;
                state.serialize_field("type", "Array")?;
                state.serialize_field("value", array)?;
                state.end()
            }
        }
    }
}


// create a test for serialization of command
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocaast_serialize() {
        let mut attributes = HashMap::new();
        let mut properties = HashMap::new();
        let mut person = HashMap::new();
        person.insert("name".to_string(), NestedValue::Value("Text".to_string()));

        attributes.insert("test".to_string(), NestedValue::Value("test".to_string()));
        properties.insert("test".to_string(), NestedValue::Value("test".to_string()));
        attributes.insert("person".to_string(), NestedValue::Object(person));
        let command = Command::Add(
            ObjectKind::CaptureBase,
            ObjectContent::CaptureBase(CaptureBaseContent {
                attributes: Some(attributes),
                properties: Some(properties),
            }),
        );
        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        let serialized = serde_json::to_string(&ocaast).unwrap();
        assert_eq!(
            serialized,
            r#"{"version":"1.0","commands":[{"type":"ADD","objectKind":"CaptureBase","attributes":{"test":"test"},"properties":{"test":"test"}}]}"#
        );
    }
}
