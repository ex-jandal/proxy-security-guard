use axum::{
    body::Body, 
    extract::{FromRequest, 
        Multipart,
        Request,
        State
    },
    http::{
        HeaderMap, 
        Method, 
        StatusCode, 
        Uri
    }, 
    response::IntoResponse
};
use reqwest::{Client, header, multipart::{Form, Part}};
use tracing::{info, debug};

use crate::{crypto::hashing::verify_signature, director::forward_multipart};
use crate::crypto::signature::sign_artwork;
use crate::director::forward_json;

pub async fn proxy_handler(
    State(client): State<Client>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    req: Request<Body>,
) -> impl IntoResponse {
    info!(" Receive a New Request: {}", uri.path());

    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let signature = headers
        .get("x-psg-signature")
        .and_then(|h| h.to_str().ok());

    if content_type.contains("multipart/form-data") {
        match Multipart::from_request(req, &()).await {
            Ok(mut multipart) => {
                let mut new_form = Form::new();
                let mut file_bytes: Vec<u8> = Vec::new();
                let mut file_name: Option<String> = None;
                let mut file_mime: Option<String> = None;

                debug!("request = {:#?}", multipart);
                
                // Find file
                while let Ok(Some(field)) = multipart.next_field().await {
                    let name = field.name().unwrap_or("file").to_string();
                    
                    if name == "file" || field.file_name().is_some() {
                        // INFO: This is our artwork file
                        file_name = field.file_name().map(|s| s.to_string());
                        file_mime = field.content_type().map(|s| s.to_string());
                        file_bytes = field.bytes().await.unwrap_or_default().to_vec();
                    } else {
                        // INFO: This is just text metadata (title, description, etc.)
                        let data = field.bytes().await.unwrap_or_default();
                        let text_val = String::from_utf8_lossy(&data).to_string();
                        new_form = new_form.text(name, text_val);
                    }
                }

                match signature {
                    Some(sig) if verify_signature(&file_bytes, sig) => {
                        info!("  Integrity Verified for artwork file");
                    }
                    _ => {
                        info!("  Signature mismatch or missing!");
                        return (StatusCode::UNAUTHORIZED, "Invalid Integrity Signature").into_response();
                    }
                }

                debug!("file name is a {}", file_name.clone().unwrap_or("Null".to_string()));
                debug!("file mime is a {}", file_mime.clone().unwrap_or("Null".to_string()));

                let mut final_headers = headers.clone();
                if uri.path().contains("/post/create") && !file_bytes.is_empty() {
                    // Sign ONLY the raw file bytes
                    let copyright_sig = sign_artwork(&file_bytes);
                    
                    final_headers.insert(
                        "X-PSG-Copyright-Seal",
                        copyright_sig.parse().unwrap()
                    );
                    info!("󰏘  Artwork notarized with seal: {}...", &copyright_sig[0..10]);
                }

                debug!("the new form = {:#?}", new_form);

                // Rebuild the file part for the backend
                if !file_bytes.is_empty() {
                    let mut part = Part::bytes(file_bytes.clone());
                    if let Some(name) = file_name { part = part.file_name(name); }
                    if let Some(mime) = file_mime { part = part.mime_str(&mime).unwrap(); }
                    new_form = new_form.part("file", part);
                }

                debug!("the new form = {:#?}", new_form);

                // Forward to NestJS
                let path_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");
                let target_url = format!("http://127.0.0.1:3000{}", path_query);

                info!("󰅑 End of the Request");

                return forward_multipart(method, target_url, headers, new_form, client).await;
            }
            Err(_) => return StatusCode::BAD_REQUEST.into_response(),
        }
    }

    else if content_type.contains("application/json") {
        let bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap_or_default();
        
        match signature {
            Some(sig) if verify_signature(&bytes, signature.unwrap_or("")) => {
                info!("  Integrity Verified json request");
            }
            _ => {
                info!("  Signature mismatch or missing!");
                return (StatusCode::UNAUTHORIZED, "Invalid Integrity Signature").into_response();
            }
        }

        // Redirect to NestJS
        let target_url = format!("http://127.0.0.1:3000{}", uri.path());

        info!("󰅑 Finish Request");
        return forward_json(method, target_url, headers, bytes, client).await;
    }

    (StatusCode::BAD_REQUEST, "the Content-Type header is not supported. Only use one of these ('application/json', 'multipart/form-data')").into_response()
}
