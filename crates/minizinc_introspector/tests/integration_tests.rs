use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::time::Duration;
use wait_timeout::ChildExt;

#[test]
fn test_narrative_minizinc_model() {
    let minizinc_exe = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc");
    let project_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc");

    if !minizinc_exe.exists() {
        panic!("MiniZinc executable not found at: {:?}", minizinc_exe);
    }

    let model_path = project_root.join("narrative_journey.mzn");

    println!("Testing MiniZinc model: {:?}", model_path);

    let mut command = Command::new(&minizinc_exe);
    command.arg(&model_path);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().expect("Failed to spawn MiniZinc command");

    match child.wait_timeout(Duration::from_secs(60)).expect("Error waiting for MiniZinc process") {
        Some(status) => {
            if !status.success() {
                let output = child.wait_with_output().expect("Failed to read MiniZinc output");
                eprintln!("--- Test Failed: {:?} ---", model_path);
                eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                panic!("MiniZinc model failed to compile or run successfully.");
            }
        },
        None => {
            // Timeout occurred, kill the child process
            child.kill().expect("Failed to kill MiniZinc process after timeout");
            child.wait().expect("Failed to wait for MiniZinc process after kill");
            panic!("MiniZinc model timed out (60s).");
        }
    }
}