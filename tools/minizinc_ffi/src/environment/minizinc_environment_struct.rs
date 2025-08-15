use crate::types::MznSolver;
use std::marker::{Send, Sync};

pub struct MiniZincEnvironment(pub *mut MznSolver);

// SAFETY:
// MiniZincEnvironment wraps a raw pointer to MznSolver.
// We are asserting that it is safe to send and share this type between threads.
// This is safe because all access to the underlying MznSolver is protected by a Mutex
// in the test environment (GLOBAL_MINIZINC_ENV).
// The MiniZinc C++ library's GC is locked during initialization and tests
// are designed to use a single global environment, preventing concurrent
// access issues to the underlying C++ state.
unsafe impl Send for MiniZincEnvironment {}
unsafe impl Sync for MiniZincEnvironment {}
