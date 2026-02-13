use axum::{
    body::Bytes,
    extract::State,
    http::{Method, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
};
use reqwest::Client;
use tracing::{info, debug};

use crate::verification::verify_signature;
use crate::director::redirect_to_backend;
use crate::DBG_MODE;

pub async fn proxy_handler(
    State(client): State<Client>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {


    info!("  Receive a New Request");
    debug!("
method  = {method:#?}
uri     = {uri:#?}
headers = {headers:#?}
body    = {body:#?}");

    // deprecated feature
    if DBG_MODE {
        println!(
"------------   New Request ------------
method  = {method:#?}
uri     = {uri:#?}
headers = {headers:#?}
body    = {body:#?}");
    }

    let path_query = uri.path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("");
    let target_url = format!("http://127.0.0.1:3000{}", path_query);

    // verifiy signatures of the requests
    let signature = headers
        .get("x-fanouni-signature")
        .and_then(|h| h.to_str().ok());

    if  method == Method::POST || 
        method == Method::PUT  || 
        method == Method::GET  {

        // deprecated feature
        if DBG_MODE {
            println!("\n---------- 󱎚  Check Signature ----------");
            println!("header_sig = {}", signature.unwrap());
        }

        match signature {
            Some(sig) if verify_signature(&body, sig) => {
                // deprecated feature
                if DBG_MODE {
                    println!("  Integrity Verified");
                }
            }
            _ => {
                // deprecated feature
                if DBG_MODE {
                    println!("  Signature mismatch or missing!");
                    println!("\nresponse_status = {:#?}", StatusCode::UNAUTHORIZED);
                    println!("---------- End of the Request ----------");
                }

                info!("  Signature mismatch or missing!");
                info!("response_status = {:#?}", StatusCode::UNAUTHORIZED);
                info!("󰅑  Finish Request");

                return (StatusCode::UNAUTHORIZED, "Invalid Integrity Signature")
                    .into_response();
            }
        }
    }


    // deprecated feature
    if DBG_MODE {
        println!("\n--------   Redirect to NestJS ---------");
    }

    // Direct to NestJS
    let response = redirect_to_backend(method, target_url, headers, body, client).await;
    info!("󰅑  Finish Request");

    response
}
