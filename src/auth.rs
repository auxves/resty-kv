use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, Response, StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use subtle::ConstantTimeEq;

use crate::Cli;

fn extract_token_from_header(req: &Request<Body>) -> Option<String> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())?;

    auth_header
        .split_whitespace()
        .skip(1)
        .next()
        .map(|x| x.to_owned())
}

pub async fn authenticate(req: Request<Body>, next: Next) -> Response<Body> {
    let Some(expected_token) = req.extensions().get::<Arc<Cli>>().unwrap().token.as_ref() else {
        return next.run(req).await;
    };

    let Some(token) = extract_token_from_header(&req) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if token.as_bytes().ct_eq(expected_token.as_bytes()).into() {
        next.run(req).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
