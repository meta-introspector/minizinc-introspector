//use crate::prelude::LogWriter;
use std::collections::HashMap; // Import HashMap
use crate::logger::LogWriter;
pub fn report_extracted_data(
    id_to_word: &HashMap<u32, String>, // Changed to HashMap
    embeddings: &HashMap<u32, Vec<f64>>, // Changed to HashMap
    logger: &mut LogWriter,
) -> Result<(), Box<dyn std::error::Error>> {
    logger.debug_log("\n--- Extracted Words and Embeddings ---");
    // Iterate over the HashMap directly
    for (id, word) in id_to_word.iter() {
        if let Some(embedding) = embeddings.get(id) {
            logger.debug_log(&format!("Word: {}, Embedding: {:?}", word, embedding));
        }
    }
    logger.debug_log("--------------------------------------");
    Ok(())
}
