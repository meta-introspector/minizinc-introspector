#ifndef MINIZINC_OPAQUE_TYPES_H
#define MINIZINC_OPAQUE_TYPES_H

#include <minizinc/solver_instance_base.hh>
#include <minizinc/solver.hh> // Include MznSolver

#ifdef __cplusplus
extern "C" {
#endif

// Opaque type for MiniZincModel
typedef struct MiniZincModel MiniZincModel;

// Opaque type for Item
typedef struct MiniZincItem MiniZincItem;

// Opaque type for MiniZinc::VarDeclI
typedef struct VarDeclI VarDeclI;

// Opaque type for MiniZinc::TypeInst
typedef struct TypeInst TypeInst;

// Opaque type for MiniZinc::AssignI
typedef struct AssignI AssignI;

// Opaque type for MiniZinc::ConstraintI
typedef struct ConstraintI ConstraintI;

// Opaque type for MiniZinc::IncludeI
typedef struct IncludeI IncludeI;

// Opaque type for MiniZinc::FunctionI
typedef struct FunctionI FunctionI;

// Opaque type for MiniZinc::Expression
typedef struct Expression Expression;

// Opaque type for MiniZinc::IntLit
typedef struct IntLit IntLit;

// Opaque type for MiniZinc::FloatLit
typedef struct FloatLit FloatLit;

// Opaque type for MiniZinc::SetLit
typedef struct SetLit SetLit;

// Wrapper struct for MiniZinc::MznSolver and its associated Timer
typedef struct MiniZincEnvWrapper {
    MiniZinc::MznSolver* solver;
    MiniZinc::Env env; // Assuming it also needs an Env object
} MiniZincEnvWrapper;


#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_OPAQUE_TYPES_H