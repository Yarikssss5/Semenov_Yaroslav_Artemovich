use std::convert::Infallible;

use prometheus::{Encoder, TextEncoder};
use axum::{Router as AxumRouter, body::HttpBody, extract::Request, 
  http::{Method, StatusCode}, middleware::{self, Next}, 
  response::{Html, IntoResponse, Response}, routing::get
};
use tokio::{net::TcpListener, time::Instant};

use crate::sse::sse_handler;

mod metrics;
mod sse;

struct SSEGuard;

impl SSEGuard {
  fn new() -> Self {
    metrics::HTTP_CONNECTED_SSE_CLIENTS.inc();
    Self
  }
}

impl Drop for SSEGuard {
    fn drop(&mut self) {
      metrics::HTTP_CONNECTED_SSE_CLIENTS.dec();
    }
}

async fn index(method: Method, req: Request) -> impl IntoResponse {
  let start: Instant = Instant::now();
  metrics::HTTP_REQUESTS_TOTAL.with_label_values(&[method.to_string(), req.uri().to_string()]).inc();
  let duration: f64 = start.elapsed().as_secs_f64();
  metrics::HTTP_RESPONSE_TIME_SECONDS
        .with_label_values(&[method.to_string(), req.uri().to_string()])
        .observe(duration);
  Html("<h1>Hello !</h1>").into_response()
}

async fn metrics() -> impl IntoResponse {
    let encoder: TextEncoder = TextEncoder::new();
    let mut buffer: Vec<u8> = vec![];
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    let response: (StatusCode, [(&str, &str); 1], Vec<u8>) = (
        StatusCode::OK,
        [("content-type", encoder.format_type())],
        buffer
    );
    response.into_response()
}

async fn my_main_middleware(req: Request, next: Next) -> Result<Response, Infallible> {
  let start: Instant = Instant::now();
  let duration: f64 = start.elapsed().as_secs_f64();
  let req_size: String = req.size_hint().lower().to_string();
  metrics::HTTP_REQUEST_SIZE.with_label_values(&[req.method().as_str(), &req.uri().to_string(), &req_size])
    .observe(duration);
  Ok(next.run(req).await)
}

#[tokio::main]
async fn main() {
  let rt: AxumRouter<_> = AxumRouter::new()
    .route("/", get(index))
    .route("/metrics", get(metrics))
    .route("/sse", get(sse_handler))
    .layer(middleware::from_fn(my_main_middleware));
  let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, rt).await.unwrap();
}