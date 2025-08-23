//! # ZOS Bootstrap - Main Entry Point
//!
//! This crate serves as the main entry point for the ZOS (Zero Ontology System) bootstrap process.
//!
//! For comprehensive documentation on the project's vision, architecture, and generated artifacts,
//! please refer to the following documents:
//!
//! ## Project Documentation
//! - [Conversation Summary and Project Vision](docs/conversation_summary_and_vision.md)
//! - [Code, Documentation, Index, and Gemini Memory Update Procedure](docs/sops/code_doc_update_sop.md)
//! - [Rust Link Verification Tool Design](docs/rust_link_verifier_design.md)
//! - [Git to MiniZinc Data Tool Design](docs/git_to_minizinc_data_tool_design.md)
//! - [Gemini Self-Model Integration Proposal](docs/gemini_self_model_integration_proposal.md)
//! - [Deep Bootstrapping and Formal Verification Strategy](docs/deep_bootstrap_verification_strategy.md)
//!
//! ## MiniZinc Models
//! - [Combinatorial Topologies](combinatorial_topologies.mzn)
//! - [Development Path Optimizer](development_path_optimizer.mzn)
//! - [Universal Bootstrap Gödel Number](universal_bootstrap_godel.mzn)
//! - [Deep Bootstrap Chain](deep_bootstrap_chain.mzn)
//!

use zos_bootstrap::command_handlers::handle_command_dispatch;
use zos_bootstrap::utils::error::Result;

fn main() -> Result<()> {
    handle_command_dispatch()
}