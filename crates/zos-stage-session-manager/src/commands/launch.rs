use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use std::fs;
use crate::utils;
use crate::args::LaunchArgs;
use chrono::{Local, Datelike, Timelike};

pub fn handle_launch_command(serialized_args: &str) -> Result<(), String> {
    eprintln!("Starting session manager stage...");

    let args: LaunchArgs = serde_json::from_str(serialized_args)
        .map_err(|e| format!("Failed to deserialize LaunchArgs: {}", e))?;

    let project_root = utils::determine_project_root()?;
    let asciicast_path = project_root.join("docs").join("asciicast_session.cast");

    if args.mode == Some("tmux".to_string()) {
        if args.inside == Some("gemini".to_string()) {
            return _handle_tmux_gemini_launch(&args);
        } else if args.inside == Some("miniact".to_string()) && args.via == Some("doh".to_string()) {
            return _handle_miniact_simulation(&args, &project_root);
        }
    }

    if args.background_detached {
        return _handle_background_detached_launch(&args);
    }

    if args.record_session {
        return _handle_asciinema_recording(&args, &asciicast_path);
    }

    _handle_additional_gemini_instances(&args)?;

    eprintln!("Session manager stage completed successfully.");
    Ok(())
}

fn _handle_tmux_gemini_launch(args: &LaunchArgs) -> Result<(), String> {
    let session_name = format!("gemini-session-{}", Local::now().format("%Y%m%d-%H%M%S"));

    eprintln!("Launching Gemini CLI in tmux session: {}", session_name);

    let full_gemini_command = _build_gemini_command_args(args);

    Command::new("tmux")
        .arg("new-session")
        .arg("-d")
        .arg("-s")
        .arg(&session_name)
        .status()
        .map_err(|e| format!("Failed to create tmux session: {}", e))?;

    Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(&session_name)
        .arg(&full_gemini_command)
        .arg("Enter")
        .status()
        .map_err(|e| format!("Failed to send command to tmux session: {}\n", e))?;

    eprintln!("Gemini CLI launched in tmux session: {}", session_name);
    Ok(())
}

fn _handle_background_detached_launch(args: &LaunchArgs) -> Result<(), String> {
    if args.gemini_instances != 1 || args.record_session {
        return Err("Cannot use --background-detached with multiple instances or session recording.".to_string());
    }
    eprintln!("Launching single Gemini instance in background, detached...");
    let shell_command = "nohup gemini --model gemini-2.5-flash --checkpointing=true > /dev/null 2>&1 &";
    eprintln!("Executing: {}", shell_command);

    let status = Command::new("bash")
        .arg("-c")
        .arg(shell_command)
        .status()
        .map_err(|e| format!("Failed to launch detached Gemini instance: {}", e))?;

    if !status.success() {
        return Err(format!("Detached Gemini instance launch failed with status: {:?}\n", status.code()));
    }
    eprintln!("Detached Gemini instance launched successfully.");
    Ok(())
}

fn _handle_asciinema_recording(args: &LaunchArgs, asciicast_path: &PathBuf) -> Result<(), String> {
    eprintln!("Recording session to: {:?}", asciicast_path);
    let shell_command = format!(
        "asciinema rec {} --command 'gemini --model gemini-2.5-flash --checkpointing=true'",
        asciicast_path.to_string_lossy()
    );
    eprintln!("Executing: {}", shell_command);

    let status = Command::new("bash")
        .arg("-c")
        .arg(&shell_command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to execute asciinema with gemini: {}", e))?;

    if !status.success() {
        return Err(format!("Asciinema/Gemini exited with non-zero status: {:?}\n", status.code()));
    }
    if args.gemini_instances == 1 {
        eprintln!("Session manager stage completed successfully.");
        Ok(())
    } else {
        Ok(())
    }
}

fn _handle_miniact_simulation(args: &LaunchArgs, project_root: &PathBuf) -> Result<(), String> {
    let workflow_file_in_repo = args.workflow_file_in_repo.clone().ok_or("Workflow file path in repository not provided for MiniAct simulation.".to_string())?;
    let target_repo_url = args.target_repo_url.clone().ok_or("Target repository URL not provided for MiniAct simulation.".to_string())?;

    eprintln!("Simulating GHA workflow with MiniAct in tmux...");

    let miniact_project_path = project_root.join("crates").join("mini-act");

    let mut miniact_command_parts: Vec<String> = Vec::new();
    miniact_command_parts.push(format!("cd {} && ./target/debug/mini-act", miniact_project_path.to_string_lossy()));
    miniact_command_parts.push(format!("--task {}", workflow_file_in_repo));
    miniact_command_parts.push(format!("--target-repo-url {}", target_repo_url));

    let full_miniact_command = miniact_command_parts.join(" ");

    let session_name = "miniact-gha-sim";
    eprintln!("Launching MiniAct in tmux session: {}", session_name);

    Command::new("tmux")
        .arg("new-session")
        .arg("-d")
        .arg("-s")
        .arg(&session_name)
        .status()
        .map_err(|e| format!("Failed to create tmux session: {}", e))?;

    Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(&session_name)
        .arg(&full_miniact_command)
        .arg("Enter")
        .status()
        .map_err(|e| format!("Failed to send command to tmux session: {}\n", e))?;

    eprintln!("MiniAct GHA simulation launched in tmux session: {}", session_name);
    Ok(())
}

fn _handle_additional_gemini_instances(args: &LaunchArgs) -> Result<(), String> {
    for i in 0..args.gemini_instances {
        if args.record_session && i == 0 {
            continue;
        }
        eprintln!("Launching Gemini instance {}/{}", i + 1, args.gemini_instances);
        let mut command = Command::new("gemini");
        command.arg("--model").arg("gemini-2.5-flash");
        command.arg("--checkpointing=true");
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
        command.spawn()
            .map_err(|e| format!("Failed to launch Gemini instance {}: {}", i + 1, e));
    }
    Ok(())
}

fn _build_gemini_command_args(args: &LaunchArgs) -> String {
    let mut gemini_command_parts: Vec<String> = Vec::new();

    let gemini_cli_executable = args.gemini_cli_path.clone().unwrap_or_else(|| "gemini".to_string());
    gemini_command_parts.push(gemini_cli_executable);

    if let Some(model) = args.model.clone() {
        gemini_command_parts.push(format!("--model {}", model));
    }
    if let Some(prompt) = args.prompt.clone() {
        gemini_command_parts.push(format!("--prompt '{}'", prompt));
    }
    if let Some(prompt_interactive) = args.prompt_interactive.clone() {
        gemini_command_parts.push(format!("--prompt-interactive '{}'", prompt_interactive));
    }
    if let Some(sandbox) = args.sandbox {
        gemini_command_parts.push(format!("--sandbox {}", sandbox));
    }
    if let Some(sandbox_image) = args.sandbox_image.clone() {
        gemini_command_parts.push(format!("--sandbox-image {}", sandbox_image));
    }
    if args.debug {
        gemini_command_parts.push("--debug".to_string());
    }
    if args.all_files {
        gemini_command_parts.push("--all-files".to_string());
    }
    if args.show_memory_usage {
        gemini_command_parts.push("--show-memory-usage".to_string());
    }
    if args.yolo {
        gemini_command_parts.push("--yolo".to_string());
    }
    if let Some(approval_mode) = args.approval_mode.clone() {
        gemini_command_parts.push(format!("--approval-mode {}", approval_mode));
    }
    if let Some(telemetry) = args.telemetry {
        gemini_command_parts.push(format!("--telemetry {}", telemetry));
    }
    if let Some(telemetry_target) = args.telemetry_target.clone() {
        gemini_command_parts.push(format!("--telemetry-target {}", telemetry_target));
    }
    if let Some(telemetry_otlp_endpoint) = args.telemetry_otlp_endpoint.clone() {
        gemini_command_parts.push(format!("--telemetry-otlp-endpoint {}", telemetry_otlp_endpoint));
    }
    if let Some(telemetry_log_prompts) = args.telemetry_log_prompts {
        gemini_command_parts.push(format!("--telemetry-log-prompts {}", telemetry_log_prompts));
    }
    if let Some(telemetry_outfile) = args.telemetry_outfile.clone() {
        gemini_command_parts.push(format!("--telemetry-outfile {}", telemetry_outfile));
    }
    if args.checkpointing {
        gemini_command_parts.push("--checkpointing".to_string());
    }
    if let Some(experimental_acp) = args.experimental_acp {
        gemini_command_parts.push(format!("--experimental-acp {}", experimental_acp));
    }
    if !args.allowed_mcp_server_names.is_empty() {
        gemini_command_parts.push(format!("--allowed-mcp-server-names {}", args.allowed_mcp_server_names.join(",")));
    }
    if !args.extensions.is_empty() {
        gemini_command_parts.push(format!("--extensions {}", args.extensions.join(",")));
    }
    if let Some(list_extensions) = args.list_extensions {
        gemini_command_parts.push(format!("--list-extensions {}", list_extensions));
    }
    if let Some(proxy) = args.proxy.clone() {
        gemini_command_parts.push(format!("--proxy {}", proxy));
    }
    if !args.include_directories.is_empty() {
        gemini_command_parts.push(format!("--include-directories {}", args.include_directories.join(",")));
    }
    if let Some(version) = args.version {
        gemini_command_parts.push(format!("--version {}", version));
    }

    gemini_command_parts.join(" ")
}