use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;

pub async fn request_id(mut req: Request, next: Next) -> Response {
    // add requset id to req
    let request_id = req.headers().get(super::REQUEST_ID_HEADER);
    let id = match request_id {
        Some(v) => Some(v.clone()),
        None => {
            // new request id
            let new_id = uuid::Uuid::now_v7().to_string();
            match HeaderValue::from_str(&new_id) {
                Ok(v) => {
                    req.headers_mut()
                        .insert(super::REQUEST_ID_HEADER, v.clone());
                    Some(v)
                }
                Err(e) => {
                    warn!("parse generated request id failed: {}", e);
                    None
                }
            }
        }
    };

    let mut res = next.run(req).await;
    let Some(id) = id else {
        return res;
    };
    res.headers_mut().insert(super::REQUEST_ID_HEADER, id);
    res
}
