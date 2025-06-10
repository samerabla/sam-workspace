use axum::{
    extract::Query,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge,
    RedirectUrl, TokenUrl,
};
use reqwest::{Client, Url};
use sam_error::SamError;
use std::{collections::HashMap, net::SocketAddr};

type SamClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

fn Oauth2Client() -> Result<SamClient, SamError> {
    let dev_host = dotenvy::var("DEV_HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("DEV_HOST".to_string()))?;
    let dist_host = dotenvy::var("HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("HOST".to_string()))?;
    let host = match cfg!(debug_assertions) {
        true => dev_host,
        false => dist_host,
    };
    let client_id = dotenvy::var("GOOGLE_CLIENT_ID")
        .map_err(|_| SamError::MissingEnviromentVariable("GOOGLE_CLIENT_ID".to_string()))?;
    let secret = dotenvy::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| SamError::MissingEnviromentVariable("GOOGLE_CLIENT_SECRET".to_string()))?;

    let google_client_id: ClientId = ClientId::new(client_id);
    let google_client_secret: ClientSecret = ClientSecret::new(secret);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url: TokenUrl = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid token endpoint URL");

    // You should have added this url in your google cloud console
    // Navigate to: https://console.cloud.google.com/ > "APIs & Services" > "Credentials" > "Authorized redirect URIs"
    let redir_url = host + "/users/login/google/callback";
    let redirect_url: RedirectUrl = RedirectUrl::new(redir_url).expect("Invalid redirect URL");

    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_token_uri(token_url)
        .set_auth_uri(auth_url)
        .set_redirect_uri(redirect_url);
    Ok(client)
}

// Handler to start the OAuth2 login process
pub async fn handler() -> Redirect {
    match Oauth2Client() {
        Ok(client) => {
            // Generate a PKCE challenge.
            let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
            let (auth_url, _csrf_token) = client
                .authorize_url(oauth2::CsrfToken::new_random)
                .add_scope(oauth2::Scope::new("email".to_string()))
                .add_scope(oauth2::Scope::new("profile".to_string()))
                //.set_pkce_challenge(pkce_challenge)
                .url();
            println!("auth_url: {}", auth_url);
            println!("CsrfToken: {:#?}", _csrf_token);
            Redirect::to(auth_url.as_str())
        }
        Err(e) => {
            println!("Error: {}", e);
            Redirect::to("/internal-error")
        }
    }
}

// TODO:
// This handler should return Response
// Handler for the OAuth2 callback
pub async fn callback(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    match Oauth2Client() {
        Ok(client) => {
            let http_client = reqwest::ClientBuilder::new()
                // Following redirects opens the client up to SSRF vulnerabilities.
                // This prevents redirects
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Client should build");

            // Now you can trade it for an access token.
            // Send a request with the code and the pkce verifier to get a token
            let token_result = client
                .exchange_code(AuthorizationCode::new(params.get("code").unwrap().clone()))
                // Set the PKCE code verifier.
                //.set_pkce_verifier(pkce_verifier)
                .request_async(&http_client)
                .await;
            match token_result {
                Ok(token) => {
                    use oauth2::TokenResponse;
                    let access_token = token.access_token().secret();
                    let user_info = Client::new()
                        .get("https://www.googleapis.com/oauth2/v3/userinfo")
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await
                        .unwrap()
                        .json::<serde_json::Value>()
                        .await
                        .unwrap();

                    Html(format!(
                        "<h1>Logged in successfully!</h1><pre>{:#?}</pre>",
                        user_info
                    ))
                }
                Err(e) => Html(format!("<h1>Error: {}</h1>", e)),
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            Html(format!("<h1>err!</h1><pre>{:#?}</pre>", e))
        }
    }
}
