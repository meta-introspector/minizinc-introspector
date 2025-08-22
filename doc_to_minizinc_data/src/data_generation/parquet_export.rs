use std::fs::File;
use std::sync::Arc;
use anyhow::Result;

use arrow::array::{ArrayRef, Float64Array, StringArray, ListArray, UInt32Array};
use arrow::buffer::OffsetBuffer;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

use std::collections::HashMap;

pub fn export_embeddings_to_parquet(
    id_to_word: &HashMap<u32, String>,
    embeddings: &HashMap<u32, Vec<f64>>,
    output_dir: &std::path::Path,
) -> Result<()> {
    println!("📦 Exporting embeddings to Parquet...");

    // 1. Define Arrow Schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::UInt32, false),
        Field::new("word", DataType::Utf8, false),
        Field::new("embedding", DataType::List(Arc::new(Field::new("item", DataType::Float64, false))), false),
    ]));

    // 2. Prepare data for RecordBatch
    let mut ids = Vec::new();
    let mut words = Vec::new();
    let mut embedding_values = Vec::new();
    let mut embedding_offsets = vec![0i32];
    let mut current_offset = 0i32;

    let mut sorted_ids: Vec<u32> = id_to_word.keys().cloned().collect();
    sorted_ids.sort_unstable();

    for id in sorted_ids {
        ids.push(id);
        if let Some(word) = id_to_word.get(&id) {
            words.push(Some(word.as_str()));
            if let Some(embedding) = embeddings.get(&id) {
                embedding_values.extend_from_slice(embedding);
                current_offset += embedding.len() as i32;
            } else {
                // Handle case where embedding might be missing (though it shouldn't if logic is sound)
                current_offset += 0;
            }
            embedding_offsets.push(current_offset);
        }
    }

    let id_array = UInt32Array::from(ids);
    let word_array = StringArray::from(words);
    let embedding_values_array = Float64Array::from(embedding_values);
    let embedding_list_array = ListArray::new(
        Arc::new(Field::new("item", DataType::Float64, false)),
        OffsetBuffer::new(embedding_offsets.into()),
        Arc::new(embedding_values_array),
        None,
    );

    let arrays: Vec<ArrayRef> = vec![
        Arc::new(id_array),
        Arc::new(word_array),
        Arc::new(embedding_list_array),
    ];

    let record_batch = RecordBatch::try_new(schema.clone(), arrays)?;

    // 3. Write to Parquet file
    let output_file_path = output_dir.join("word_embeddings.parquet");
    let file = File::create(&output_file_path)?;

    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

    writer.write(&record_batch)?;
    writer.close()?;

    println!("✅ Embeddings exported to {output_file_path:?}");

    Ok(())
}
