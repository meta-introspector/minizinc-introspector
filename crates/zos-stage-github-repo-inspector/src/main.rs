use clap::{Parser, Subcommand};
use tokio;
use octocrab::{Octocrab, models::workflows::{Run, Job}, models::Artifact, models::RunId, Result};


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


async fn fetch_workflow_runs(octocrab: &Octocrab, owner: &str, repo: &str) -> Result<Vec<Run>> {
    eprintln!("Fetching workflow runs for {}/{}", owner, repo);
    let page = octocrab.runs().list().owner(owner.to_string()).repo(repo.to_string()).send().await?;
    Ok(page.items)
}

async fn fetch_jobs_for_run(octocrab: &Octocrab, owner: &str, repo: &str, run_id: u64) -> Result<Vec<Job>> {
    eprintln!("Fetching jobs for run {} in {}/{}", run_id, owner, repo);
    let page = octocrab.workflows(owner, repo).list_jobs(RunId(run_id)).send().await?;
    Ok(page.items)
}

async fn fetch_artifacts_for_run(octocrab: &Octocrab, owner: &str, repo: &str, run_id: u64) -> Result<Vec<Artifact>> {
    eprintln!("Fetching artifacts for run {} in {}/{}", run_id, owner, repo);
    let page = octocrab.artifacts().list().owner(owner.to_string()).repo(repo.to_string()).run_id(run_id).send().await?;
    Ok(page.items)
}

async fn fetch_job_logs(octocrab: &Octocrab, owner: &str, repo: &str, job_id: u64) -> Result<String> {
    eprintln!("Fetching logs for job {} in {}/{}", job_id, owner, repo);
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/jobs/{}/logs",
        owner, repo, job_id
    );
    let response = octocrab.get(url, None::<&()>)
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let octocrab = Octocrab::builder()
        .personal_token(cli.token.clone()) // Use clone as cli.token is moved into octocrab
        .build()?;

    match &cli.command {
        Commands::Runs => {
            let runs = fetch_workflow_runs(&octocrab, &cli.owner, &cli.repo).await?;
            eprintln!("--- Workflow Runs for {}/{} ---", cli.owner, cli.repo);
            for run in runs {
                println!("ID: {}", run.id);
                println!("Name: {}", run.name.unwrap_or_default());
                println!("Status: {:?}", run.status); // Use {:?} for Status enum
                println!("Conclusion: {:?}", run.conclusion); // Use {:?} for Option<Conclusion>
                println!("URL: {}", run.html_url);
                println!("Created At: {}", run.created_at);
                println!("---------------------");
            }
        },
        Commands::Jobs { run_id } => {
            let jobs = fetch_jobs_for_run(&octocrab, &cli.owner, &cli.repo, *run_id).await?;
            eprintln!("--- Jobs for Run {} in {}/{} ---", run_id, cli.owner, cli.repo);
            for job in jobs {
                println!("ID: {}", job.id);
                println!("Name: {}", job.name);
                println!("Status: {:?}", job.status); // Use {:?} for Status enum
                println!("Conclusion: {:?}", job.conclusion); // Use {:?} for Option<Conclusion>
                println!("URL: {}", job.html_url);
                println!("Started At: {}", job.started_at);
                println!("Completed At: {}", job.completed_at.map(|dt| dt.to_string()).unwrap_or_default()); // Convert DateTime to String
                if let Some(steps) = job.steps {
                    println!("  Steps:");
                    for step in steps {
                        println!("    - {}: {} ({:?})", step.number, step.name, step.status); // Use {:?} for Status enum
                    }
                }
                println!("---------------------");
            }
        },
        Commands::Artifacts { run_id } => {
            let artifacts = fetch_artifacts_for_run(&octocrab, &cli.owner, &cli.repo, *run_id).await?;
            eprintln!("--- Artifacts for Run {} in {}/{} ---", run_id, cli.owner, cli.repo);
            for artifact in artifacts {
                println!("ID: {}", artifact.id);
                println!("Name: {}", artifact.name);
                println!("Size: {} bytes", artifact.size_in_bytes);
                println!("Download URL: {}", artifact.archive_download_url);
                println!("Created At: {}", artifact.created_at);
                println!("---------------------");
            }
        },
        Commands::JobLogs { job_id } => {
            let logs = fetch_job_logs(&octocrab, &cli.owner, &cli.repo, *job_id).await?;
            eprintln!("--- Logs for Job {} in {}/{}", job_id, cli.owner, cli.repo);
            println!("{}", logs);
            eprintln!("---------------------");
        },
    }

    Ok(())
}
