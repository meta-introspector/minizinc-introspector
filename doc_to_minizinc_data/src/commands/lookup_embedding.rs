use crate::prelude::*;

pub fn handle_lookup_embedding_command(word: String) -> Result<()> {
    println!("Looking up embedding for word: \"{word}\"");
    let parquet_file_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/huggingface/word_embeddings.parquet");
    let file = File::open(&parquet_file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;

    let mut found_embedding = false;
    for batch_result in reader {
        let record_batch = batch_result?;
        let word_column = record_batch.column_by_name("word").ok_or_else(|| anyhow::anyhow!("Word column not found"))?;
        let embedding_column = record_batch.column_by_name("embedding").ok_or_else(|| anyhow::anyhow!("Embedding column not found"))?;

        let word_array = word_column.as_any().downcast_ref::<StringArray>().ok_or_else(|| anyhow::anyhow!("Word column is not StringArray"))?;
        let embedding_list_array = embedding_column.as_any().downcast_ref::<ListArray>().ok_or_else(|| anyhow::anyhow!("Embedding column is not ListArray"))?;

        for i in 0..word_array.len() {
            let current_word = word_array.value(i).to_string(); // Removed strip_suffix
            println!("DEBUG: current_word = \"{current_word}\""); // Debug print
            if current_word == word {
                let embedding_values = embedding_list_array.value(i);
                let float_array = embedding_values.as_any().downcast_ref::<Float64Array>().ok_or_else(|| anyhow::anyhow!("Embedding list element is not Float64Array"))?;
                
                print!("Embedding for \"{word}\": [");
                for j in 0..float_array.len() {
                    print!("{}", float_array.value(j));
                    if j < float_array.len() - 1 {
                        print!( ", ");
                    }
                }
                println!("]");
                found_embedding = true;
                break;
            }
        }
        if found_embedding {
            break;
        }
    }

    if !found_embedding {
        println!("Embedding for \"{word}\" not found.");
    }

    Ok(())
}
