#[derive(Debug)]
pub enum MiniZincExpressionId {
    IntLit = 1,
    FloatLit = 2,
    SetLit = 3,
    BoolLit = 4,
    StringLit = 5,
    Id = 6,
    Anon = 7,
    ArrayLit = 8,
    ArrayAccess = 9,
    FieldAccess = 10,
    Comp = 11,
    Ite = 12,
    BinOp = 13,
    UnOp = 14,
    Call = 15,
    VarDecl = 16,
    Let = 17,
    TypeInst = 18,
    TypeInstId = 19,
    Unknown = 0,
}

impl From<i32> for MiniZincExpressionId {
    fn from(id: i32) -> Self {
        match id {
            1 => MiniZincExpressionId::IntLit,
            2 => MiniZincExpressionId::FloatLit,
            3 => MiniZincExpressionId::SetLit,
            4 => MiniZincExpressionId::BoolLit,
            5 => MiniZincExpressionId::StringLit,
            6 => MiniZincExpressionId::Id,
            7 => MiniZincExpressionId::Anon,
            8 => MiniZincExpressionId::ArrayLit,
            9 => MiniZincExpressionId::ArrayAccess,
            10 => MiniZincExpressionId::FieldAccess,
            11 => MiniZincExpressionId::Comp,
            12 => MiniZincExpressionId::Ite,
            13 => MiniZincExpressionId::BinOp,
            14 => MiniZincExpressionId::UnOp,
            15 => MiniZincExpressionId::Call,
            16 => MiniZincExpressionId::VarDecl,
            17 => MiniZincExpressionId::Let,
            18 => MiniZincExpressionId::TypeInst,
            19 => MiniZincExpressionId::TypeInstId,
            _ => MiniZincExpressionId::Unknown,
        }
    }
}