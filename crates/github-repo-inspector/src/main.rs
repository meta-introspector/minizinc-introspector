use clap::{Parser, Subcommand};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use serde::Deserialize;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// GitHub repository owner (e.g., 'octocat')
    #[arg(short, long)]
    owner: String,
    /// GitHub repository name (e.g., 'Spoon-Knife')
    #[arg(short, long)]
    repo: String,
    /// GitHub Personal Access Token (PAT) with 'repo' scope
    #[arg(short, long)]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists workflow runs
    Runs,
    /// Lists jobs for a specific workflow run
    Jobs {
        #[arg(long)]
        run_id: u64,
    },
    /// Lists artifacts for a specific workflow run
    Artifacts {
        #[arg(long)]
        run_id: u64,
    },
    /// Fetches logs for a specific job
    JobLogs {
        #[arg(long)]
        job_id: u64,
    },
}

#[derive(Debug, Deserialize)]
struct WorkflowRuns {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    id: u64,
    name: Option<String>,
    status: String,
    conclusion: Option<String>,
    html_url: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct Jobs {
    jobs: Vec<Job>,
}

#[derive(Debug, Deserialize)]
struct Job {
    id: u64,
    name: String,
    status: String,
    conclusion: Option<String>,
    html_url: String,
    started_at: String,
    completed_at: Option<String>,
    steps: Option<Vec<Step>>,
}

#[derive(Debug, Deserialize)]
struct Step {
    name: String,
    status: String,
    conclusion: Option<String>,
    number: u64,
    started_at: Option<String>,
    completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Artifacts {
    artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize)]
struct Artifact {
    id: u64,
    name: String,
    size_in_bytes: u64,
    archive_download_url: String,
    created_at: String,
}

async fn fetch_workflow_runs(client: &reqwest::Client, owner: &str, repo: &str, token: &str) -> Result<WorkflowRuns, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("github-actions-inspector"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);

    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs",
        owner, repo
    );

    eprintln!("Fetching workflow runs from: {}", url);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<WorkflowRuns>()
        .await?;
    Ok(response)
}

async fn fetch_jobs_for_run(client: &reqwest::Client, owner: &str, repo: &str, run_id: u64, token: &str) -> Result<Jobs, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("github-actions-inspector"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);

    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs/{}/jobs",
        owner, repo, run_id
    );

    eprintln!("Fetching jobs for run {} from: {}", run_id, url);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<Jobs>()
        .await?;
    Ok(response)
}

async fn fetch_artifacts_for_run(client: &reqwest::Client, owner: &str, repo: &str, run_id: u64, token: &str) -> Result<Artifacts, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("github-actions-inspector"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);

    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runs/{}/artifacts",
        owner, repo, run_id
    );

    eprintln!("Fetching artifacts for run {} from: {}", run_id, url);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<Artifacts>()
        .await?;
    Ok(response)
}

async fn fetch_job_logs(client: &reqwest::Client, owner: &str, repo: &str, job_id: u64, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("github-actions-inspector"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);
    headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("application/vnd.github.v3.raw")); // Request raw logs

    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/jobs/{}/logs",
        owner, repo, job_id
    );

    eprintln!("Fetching logs for job {} from: {}", job_id, url);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = reqwest::Client::new();

    match &cli.command {
        Commands::Runs => {
            let response = fetch_workflow_runs(&client, &cli.owner, &cli.repo, &cli.token).await?;
            eprintln!("--- Workflow Runs ---");
            for run in response.workflow_runs {
                println!("ID: {}", run.id);
                println!("Name: {}", run.name.unwrap_or_else(|| "N/A".to_string()));
                println!("Status: {}", run.status);
                println!("Conclusion: {}", run.conclusion.unwrap_or_else(|| "N/A".to_string()));
                println!("URL: {}", run.html_url);
                println!("Created At: {}", run.created_at);
                println!("---------------------");
            }
        },
        Commands::Jobs { run_id } => {
            let response = fetch_jobs_for_run(&client, &cli.owner, &cli.repo, *run_id, &cli.token).await?;
            eprintln!("--- Jobs for Run {} ---", run_id);
            for job in response.jobs {
                println!("ID: {}", job.id);
                println!("Name: {}", job.name);
                println!("Status: {}", job.status);
                println!("Conclusion: {}", job.conclusion.unwrap_or_else(|| "N/A".to_string()));
                println!("URL: {}", job.html_url);
                println!("Started At: {}", job.started_at);
                println!("Completed At: {}", job.completed_at.unwrap_or_else(|| "N/A".to_string()));
                if let Some(steps) = job.steps {
                    println!("  Steps:");
                    for step in steps {
                        println!("    - {}: {} ({})", step.number, step.name, step.status);
                    }
                }
                println!("---------------------");
            }
        },
        Commands::Artifacts { run_id } => {
            let response = fetch_artifacts_for_run(&client, &cli.owner, &cli.repo, *run_id, &cli.token).await?;
            eprintln!("--- Artifacts for Run {} ---", run_id);
            for artifact in response.artifacts {
                println!("ID: {}", artifact.id);
                println!("Name: {}", artifact.name);
                println!("Size: {} bytes", artifact.size_in_bytes);
                println!("Download URL: {}", artifact.archive_download_url);
                println!("Created At: {}", artifact.created_at);
                println!("---------------------");
            }
        },
        Commands::JobLogs { job_id } => {
            let logs = fetch_job_logs(&client, &cli.owner, &cli.repo, *job_id, &cli.token).await?;
            eprintln!("--- Logs for Job {} ---", job_id);
            println!("{}", logs);
            eprintln!("---------------------");
        },
    }

    Ok(())
}