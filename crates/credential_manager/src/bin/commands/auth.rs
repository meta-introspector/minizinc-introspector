use clap::Subcommand;
// The following imports are for the OAuth2 logic, which is currently commented out.
// They will be re-enabled when the OAuth2 functionality is properly integrated.
// use rpassword::read_password;
// use std::sync::{Arc, Mutex};
// use oauth2::{
//     AuthUrl,
//     AuthorizationCode,
//     ClientId,
//     ClientSecret,
//     CsrfToken,
//     PkceCodeChallenge,
//     RedirectUrl,
//     Scope,
//     TokenResponse,
//     TokenUrl,
// };
// use oauth2::basic::BasicClient;
// use url::Url;

// use hyper::{
//     Body,
//     Request,
//     Response,
//     Server,
//     StatusCode,
// };
// use hyper::service::{make_service_fn, service_fn};
// use std::convert::Infallible;
// use std::net::SocketAddr;
// use tokio::sync::oneshot;
// use tokio_stream::wrappers::TcpListenerStream;
// use reqwest::Client as ReqwestClient; // Import ReqwestClient
// use oauth2::http::Version; // Import http::Version

#[derive(Subcommand, Debug)]
pub enum AuthService {
    /// Authenticates with GitHub
    Github,
}

pub async fn handle_auth_command(service: &AuthService) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    match service {
        AuthService::Github => {
            // The following code was commented out by the user.
            // It will be re-enabled and properly integrated after refactoring.
            //
            // let client_id = ClientId::new("YOUR_GITHUB_CLIENT_ID".to_string());
            // let client_secret = ClientSecret::new("YOUR_GITHUB_CLIENT_SECRET".to_string());
            // let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
            // let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

            // // Set up the config for the GitHub OAuth2 process.
            // let client = BasicClient::new(
            //     client_id,
            // )
            // .set_client_secret(client_secret)
            // .set_auth_uri(auth_url)
            // .set_token_uri(token_url)
            // .set_redirect_uri(RedirectUrl::new("http://localhost:8080/redirect".to_string())?);

            // // Generate the authorization URL to which we'll redirect the user.
            // let (authorize_url, csrf_state) = client
            //     .authorize_url(CsrfToken::new_random)
            //     .add_scope(Scope::new("repo".to_string())) // Request 'repo' scope for GitHub API access
            //     .url();

            // println!("Open this URL in your browser:");
            // println!("{}", authorize_url);

            // // Start a local server to receive the redirect
            // let (tx, rx) = oneshot::channel();
            // let tx_arc = Arc::new(Mutex::new(Some(tx))); // Wrap tx in Arc<Mutex<Option<>>>
            // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

            // let server_future = Server::bind(&addr).serve(make_service_fn(move |_conn| {
            //     let tx_arc = tx_arc.clone(); // Clone the Arc for each connection
            //     async move {
            //         Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
            //             let tx_arc = tx_arc.clone(); // Clone the Arc for each request
            //             async move {
            //                 let query = req.uri().query().unwrap_or("");
            //                 let query_params: Vec<(_, _)> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();

            //                 let code = query_params.iter().find(|(k, _)| k == "code").map(|(_, v)| v.to_owned());
            //                 let state = query_params.iter().find(|(k, _)| k == "state").map(|(_, v)| v.to_owned());

            //                 if let (Some(code), Some(state)) = (code, state) {
            //                     // Send the code and state back to the main thread
            //                     if let Some(sender) = tx_arc.lock().unwrap().take() {
            //                         let _ = sender.send((code, state));
            //                     }
            //                     Ok::<_, Infallible>(Response::new(Body::from("Authorization successful! You can close this window.")))
            //                 } else {
            //                     Ok::<_, Infallible>(Response::builder()
            //                             .status(StatusCode::BAD_REQUEST)
            //                             .body(Body::from("Missing code or state in redirect"))
            //                             .unwrap())
            //                 }
            //             }
            //         }))
            //     }
            // }));

            // println!("Waiting for redirect to http://localhost:8080/redirect...");
            // // Await both the server and the receiver. The server will stop once tx is sent.
            // let (code, state) = tokio::select! {
            //     _ = server_future => {
            //         // Server stopped, but we't get the code/state
            //         return Err("Server stopped without receiving authorization code.".into());
            //     },
            //     result = rx => {
            //         result.unwrap() // This will panic if the sender is dropped without sending
            //     }
            // };

            // // Verify CSRF state
            // if state != *csrf_state.secret() { // Corrected comparison
            //     return Err("CSRF state mismatch".into());
            // }

            // // Exchange the code for an access token
            // let token_response = client
            //     .exchange_code(AuthorizationCode::new(code))
            //     .request_async(&|request: oauth2::HttpRequest| async move {
            //         let http_client = ReqwestClient::new(); // Create a reqwest client inside the closure
            //         let reqwest_request = reqwest::Request::try_from(request)
            //             .expect("Failed to convert oauth2::HttpRequest to reqwest::Request");
            //         let reqwest_response = http_client.execute(reqwest_request).await.map_err(|e| oauth2::RequestTokenError::Other(e.to_string()))?;

            //         let status = reqwest_response.status();
            //         let headers = reqwest_response.headers().clone();
            //         let chunks = reqwest_response.bytes().await.map_err(|e| oauth2::RequestTokenError::Other(e.to_string()))?;
            //         let body = chunks.to_vec();

            //         Ok::<oauth2::http::Response<Vec<u8>>, oauth2::RequestTokenError<reqwest::Error, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>>(oauth2::http::Response::builder()
            //             .status(status)
            //             .version(Version::HTTP_11) // Use http::Version
            //             .body(body)
            //             .unwrap())
            //     })
            //     .await?;

            // let access_token = token_response.access_token().secret();
            // store_credential("github", "default", access_token)?;
            // println!("GitHub PAT stored successfully.");
            println!("GitHub OAuth authentication is currently disabled for refactoring.");
            Ok(())
        }
    }
}