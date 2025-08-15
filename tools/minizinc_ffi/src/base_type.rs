#[derive(Debug)]
pub enum MiniZincBaseType {
    Bool,
    Int,
    Float,
    Set,
    String,
    Array,
    Annotation,
    Bottom,
    Unknown,
}

impl From<i32> for MiniZincBaseType {
    fn from(id: i32) -> Self {
        match id {
            0 => MiniZincBaseType::Bool,
            1 => MiniZincBaseType::Int,
            2 => MiniZincBaseType::Float,
            3 => MiniZincBaseType::Set,
            4 => MiniZincBaseType::String,
            5 => MiniZincBaseType::Array,
            6 => MiniZincBaseType::Annotation,
            7 => MiniZincBaseType::Bottom,
            _ => MiniZincBaseType::Unknown,
        }
    }
}