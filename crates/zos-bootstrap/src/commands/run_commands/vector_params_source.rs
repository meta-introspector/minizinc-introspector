pub enum VectorParamsSource {
    /// Use a pre-existing, versioned DZN file for vector parameters.
    Version(String),
    /// Dynamically generate the vector parameters DZN file using a Rust program.
    NumVec(u32),
}
