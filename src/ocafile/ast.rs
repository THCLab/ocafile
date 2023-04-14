use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Add,
    Remove,
    Modify,
    From
}
#[derive(Debug, PartialEq)]
pub(crate) enum ObjectKind {
    CaptureBase,
    Overlay
}

#[derive(Debug, PartialEq)]
pub(crate) struct Object {
    pub(crate) kind: ObjectKind,
    pub(crate) attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InstructionData {
    From(String),
    Object(Object),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Instruction {
    pub(crate) command: Command,
    pub(crate) data: InstructionData,
}

#[derive(Debug, PartialEq)]
pub(crate) struct OCAfileAst {
    instruction_list: Vec<Instruction>
}

impl OCAfileAst {
    pub(crate) fn new() -> Self {
        OCAfileAst {
            instruction_list: Vec::new()
        }
    }
}

impl Instruction {
    pub(crate) fn new(command: Command, data: InstructionData) -> Self {
        Instruction {
            command,
            data
        }
    }
}

impl Object {
    pub(crate) fn new(kind: ObjectKind, attributes: HashMap<String, String>) -> Self {
        Object {
            kind,
            attributes
        }
    }
}