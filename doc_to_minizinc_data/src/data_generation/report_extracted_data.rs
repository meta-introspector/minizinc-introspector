use crate::logger::LogWriter;

pub fn report_extracted_data(
    id_to_word: &Vec<String>,
    embeddings: &Vec<Vec<f64>>,
    logger: &mut LogWriter,
) -> Result<(), Box<dyn std::error::Error>> {
    logger.debug_log("\n--- Extracted Words and Embeddings ---");
    for (id, word) in id_to_word.iter().enumerate() {
        logger.debug_log(&format!("Word: {}, Embedding: {:?}", word, embeddings[id]));
    }
    logger.debug_log("--------------------------------------");
    Ok(())
}
