use clap::{Parser, Subcommand};
use credential_manager::{store_credential, retrieve_credential};
use rpassword::read_password;
use std::sync::{Arc, Mutex};
use oauth2::{
    AuthUrl,
    AuthorizationCode,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
};
use oauth2::basic::BasicClient;
use url::Url;

use hyper::{
    Body,
    Request,
    Response,
    Server,
    StatusCode,
};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio_stream::wrappers::TcpListenerStream;
use reqwest::Client as ReqwestClient; // Import ReqwestClient
use oauth2::http::Version; // Import http::Version

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Stores a credential
    Store {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    },
    /// Retrieves a credential
    Retrieve {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    },
    /// Lists stored credentials (placeholder - keyring does not directly support listing)
    List,
    /// Authenticates with a service using OAuth2
    Auth {
        #[command(subcommand)]
        service: AuthService,
    },
}

#[derive(Subcommand, Debug)]
enum AuthService {
    /// Authenticates with GitHub
    Github,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Store { service, username } => {
            println!("Enter password for service '{}' and username '{}':", service, username);
            let password = read_password()?;
            store_credential(service, username, &password)?;
            println!("Credential stored successfully.");
        },
        Commands::Retrieve { service, username } => {
            match retrieve_credential(service, username) {
                Ok(password) => {
                    println!("Password for service '{}' and username '{}': {}", service, username, password);
                },
                Err(e) => {
                    eprintln!("Error retrieving credential: {}", e);
                }
            }
        },
        Commands::List => {
            println!("Listing credentials is not directly supported by the keyring crate.");
            println!("You can try retrieving specific credentials if you know the service and username.");
        },
        Commands::Auth { service } => {
            match service {
                AuthService::Github => {
                    let client_id = ClientId::new("YOUR_GITHUB_CLIENT_ID".to_string());
                    let client_secret = ClientSecret::new("YOUR_GITHUB_CLIENT_SECRET".to_string());
                    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
                    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

                    // Set up the config for the GitHub OAuth2 process.
                    let client = BasicClient::new(
                        client_id,
                    )
                    .set_client_secret(client_secret)
                    .set_auth_uri(auth_url)
                    .set_token_uri(token_url)
                    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/redirect".to_string())?);

                    // Generate the authorization URL to which we'll redirect the user.
                    let (authorize_url, csrf_state) = client
                        .authorize_url(CsrfToken::new_random)
                        .add_scope(Scope::new("repo".to_string())) // Request 'repo' scope for GitHub API access
                        .url();

                    println!("Open this URL in your browser:");
                    println!("{}", authorize_url);

                    // Start a local server to receive the redirect
                    let (tx, rx) = oneshot::channel();
                    let tx_arc = Arc::new(Mutex::new(Some(tx))); // Wrap tx in Arc<Mutex<Option<>>>
                    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

                    let server_future = Server::bind(&addr).serve(make_service_fn(move |_conn| {
                        let tx_arc = tx_arc.clone(); // Clone the Arc for each connection
                        async move {
                            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                                let tx_arc = tx_arc.clone(); // Clone the Arc for each request
                                async move {
                                    let query = req.uri().query().unwrap_or("");
                                    let query_params: Vec<(_, _)> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();

                                    let code = query_params.iter().find(|(k, _)| k == "code").map(|(_, v)| v.to_owned());
                                    let state = query_params.iter().find(|(k, _)| k == "state").map(|(_, v)| v.to_owned());

                                    if let (Some(code), Some(state)) = (code, state) {
                                        // Send the code and state back to the main thread
                                        if let Some(sender) = tx_arc.lock().unwrap().take() {
                                            let _ = sender.send((code, state));
                                        }
                                        Ok::<_, Infallible>(Response::new(Body::from("Authorization successful! You can close this window.")))
                                    } else {
                                        Ok::<_, Infallible>(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from("Missing code or state in redirect"))
                                                .unwrap())
                                    }
                                }
                            }))
                        }
                    }));

                    println!("Waiting for redirect to http://localhost:8080/redirect...");
                    // Await both the server and the receiver. The server will stop once tx is sent.
                    let (code, state) = tokio::select! {
                        _ = server_future => {
                            // Server stopped, but we didn't get the code/state
                            return Err("Server stopped without receiving authorization code.".into());
                        },
                        result = rx => {
                            result.unwrap() // This will panic if the sender is dropped without sending
                        }
                    };

                    // Verify CSRF state
                    if state != *csrf_state.secret() { // Corrected comparison
                        return Err("CSRF state mismatch".into());
                    }

                    // Exchange the code for an access token
                    let token_response = client
                        .exchange_code(AuthorizationCode::new(code))
                        .request_async(&|request: oauth2::HttpRequest| async move {
                            let http_client = ReqwestClient::new(); // Create a reqwest client inside the closure
                            let reqwest_request = reqwest::Request::try_from(request)
                                .expect("Failed to convert oauth2::HttpRequest to reqwest::Request");
                            let reqwest_response = http_client.execute(reqwest_request).await.map_err(|e| oauth2::RequestTokenError::Other(e.to_string()))?;

                            let status = reqwest_response.status();
                            let headers = reqwest_response.headers().clone();
                            let chunks = reqwest_response.bytes().await.map_err(|e| oauth2::RequestTokenError::Other(e.to_string()))?;
                            let body = chunks.to_vec();

                            Ok::<oauth2::http::Response<Vec<u8>>, oauth2::RequestTokenError<reqwest::Error, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>>(oauth2::http::Response::builder()
                                .status(status)
                                .version(Version::HTTP_11) // Use http::Version
                                .body(body)
                                .unwrap())
                        })
                        .await?;

                    let access_token = token_response.access_token().secret();
                    store_credential("github", "default", access_token)?;
                    println!("GitHub PAT stored successfully.");
                }
            }
        },
    }

    Ok(())
}
