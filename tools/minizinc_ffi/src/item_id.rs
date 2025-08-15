#[derive(Debug)]
pub enum ItemId {
    VarDecl = 1,
    Include = 2,
    Assign = 3,
    Constraint = 4,
    Solve = 5,
    Output = 6,
    Function = 7,
    Unknown = 0,
}

impl From<i32> for ItemId {
    fn from(id: i32) -> Self {
        match id {
            1 => ItemId::VarDecl,
            2 => ItemId::Include,
            3 => ItemId::Assign,
            4 => ItemId::Constraint,
            5 => ItemId::Solve,
            6 => ItemId::Output,
            7 => ItemId::Function,
            _ => ItemId::Unknown,
        }
    }
}