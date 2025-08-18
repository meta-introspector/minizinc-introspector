use std::{process, path::PathBuf};
use crate::logger::LogWriter;

pub fn run_minizinc_test(dzn_path: &PathBuf, logger: &mut LogWriter) -> Result<(), Box<dyn std::error::Error>> {
    let minizinc_executable = "/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc";
    let minizinc_model = "/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn";

    logger.log(&format!("Running MiniZinc for: {}", dzn_path.display()));
    let output = process::Command::new(minizinc_executable)
        .arg(minizinc_model)
        .arg(&dzn_path)
        .output()?;

    if output.status.success() {
        logger.log(&format!("MiniZinc successful for {}\nStdout:\n{}\nStderr:\n{}",
                             dzn_path.display(),
                             String::from_utf8_lossy(&output.stdout),
                             String::from_utf8_lossy(&output.stderr)));
    } else {
        let error_msg = format!("MiniZinc failed for {}\nStdout:\n{}\nStderr:\n{}",
                                 dzn_path.display(),
                                 String::from_utf8_lossy(&output.stdout),
                                 String::from_utf8_lossy(&output.stderr));
        logger.log(&error_msg);
        return Err(error_msg.into()); // Return an error if MiniZinc fails
    }
    Ok(())
}

