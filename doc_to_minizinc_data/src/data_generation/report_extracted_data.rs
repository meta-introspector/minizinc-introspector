//use crate::prelude::LogWriter;
use anyhow::Result;
use std::collections::HashMap; // Import HashMap
use crate::logger::LogWriter;
pub fn report_extracted_data(
    id_to_word: &HashMap<u32, String>, // Changed to HashMap
    embeddings: &HashMap<u32, Vec<f64>>, // Changed to HashMap
    logger: &mut LogWriter,
) -> Result<()> {
    logger.debug_log("\n--- Extracted Words and Embeddings ---");
    // Iterate over the HashMap directly
    for (id, word) in id_to_word.iter() {
        if let Some(embedding) = embeddings.get(id) {
            logger.debug_log(&format!("Word: {word}, Embedding: {embedding:?}"));
        }
    }
    logger.debug_log("--------------------------------------");
    Ok(())
}
