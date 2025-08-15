#[repr(C)]
pub struct MiniZincModel(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincEnvWrapper(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincSolveItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincOutputItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincAssignItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincConstraintItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincIncludeItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincFunctionItem(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincFloatLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincSetLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincBoolLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincStringLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincId(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincAnonVar(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincArrayLit(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MznSolver { _data: [u8; 0] }
#[repr(C)]
#[derive(Debug)]
pub struct MiniZincExpression(pub *mut std::os::raw::c_void);
#[repr(C)]
pub struct MiniZincVarDecl(pub *mut std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct MiniZincTypeInst(pub *mut std::os::raw::c_void);