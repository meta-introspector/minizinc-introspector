#![allow(unused_imports)] // Suppress unused import warnings for this file
use crate::prelude::*;
use arrow::datatypes::Schema;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::path::PathBuf;
use crate::utils::arrow_schema_printer::print_arrow_schema_fields;

pub fn handle_inspect_parquet_schema_command(file_path: PathBuf) -> Result<()> {
    println!("Inspecting Parquet schema for: {file_path:?}");

    let file = File::open(&file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let arrow_schema = builder.schema().clone();

    println!("Parquet Schema:");
    print_arrow_schema_fields(&arrow_schema, 0);

    Ok(())
}


