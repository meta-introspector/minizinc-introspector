pub struct MiniZincModel(pub *mut std::os::raw::c_void);
pub struct MiniZincItem(pub *mut std::os::raw::c_void);
pub struct MiniZincEnvWrapper(pub *mut std::os::raw::c_void);
pub struct MiniZincSolveItem(pub *mut std::os::raw::c_void);
pub struct MiniZincOutputItem(pub *mut std::os::raw::c_void);
pub struct MiniZincAssignItem(pub *mut std::os::raw::c_void);
pub struct MiniZincConstraintItem(pub *mut std::os::raw::c_void);
pub struct MiniZincIncludeItem(pub *mut std::os::raw::c_void);
pub struct MiniZincFunctionItem(pub *mut std::os::raw::c_void);
pub struct MiniZincFloatLit(pub *mut std::os::raw::c_void);
pub struct MiniZincSetLit(pub *mut std::os::raw::c_void);
pub struct MiniZincBoolLit(pub *mut std::os::raw::c_void);
pub struct MiniZincStringLit(pub *mut std::os::raw::c_void);
pub struct MiniZincId(pub *mut std::os::raw::c_void);
pub struct MiniZincAnonVar(pub *mut std::os::raw::c_void);
pub struct MiniZincArrayLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MznSolver { _data: [u8; 0] }
#[derive(Debug)]
pub struct MiniZincExpression(pub *mut std::os::raw::c_void);
pub struct MiniZincVarDecl(pub *mut std::os::raw::c_void);
#[derive(Debug)]
pub struct MiniZincTypeInst(pub *mut std::os::raw::c_void);