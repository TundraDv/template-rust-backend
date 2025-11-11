use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{Instrument, info_span};
use uuid::Uuid;

pub async fn tracing_middleware(request: Request, next: Next) -> Response {
    let request_id = Uuid::now_v7();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let span = info_span!(
        "request",
        request_id = %request_id,
        method = %method,
        uri = %uri
    );

    let response = next.run(request).instrument(span).await;

    let duration = start.elapsed();
    let status = response.status();

    tracing::info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        status = %status.as_u16(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );

    response
}
