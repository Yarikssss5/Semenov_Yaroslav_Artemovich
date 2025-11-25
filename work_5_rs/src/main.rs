use prometheus::{Encoder, HistogramVec, IntCounterVec, IntGauge, TextEncoder, opts, register_histogram_vec, 
  register_int_counter_vec, register_int_gauge};
use axum::{Router as AxumRouter, extract::Request, http::{Method, StatusCode}, response::{Html, IntoResponse}, routing::get};
use lazy_static::lazy_static;
use tokio::{net::TcpListener, time::Instant};


const HTTP_RESPONSE_TIME_CUSTOM_BUCKETS: &[f64; 14] = &[
    0.0005, 0.0008, 0.00085, 0.0009, 0.00095, 0.001, 0.00105, 0.0011, 0.00115, 0.0012, 0.0015,
    0.002, 0.003, 1.0,
];

lazy_static! {
   pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("my_http_requests_total", "HTTP requests total"),
        &["method", "path"]
    )
    .expect("Can't create a metric");
    pub static ref HTTP_CONNECTED_SSE_CLIENTS: IntGauge =
        register_int_gauge!(opts!("my_http_connected_sse_clients", "Connected SSE clients"))
            .expect("Can't create a metric");
    pub static ref HTTP_RESPONSE_TIME_SECONDS: HistogramVec = register_histogram_vec!(
        "my_http_response_time_seconds",
        "HTTP response times",
        &["method", "path"],
        HTTP_RESPONSE_TIME_CUSTOM_BUCKETS.to_vec()
    )
    .expect("Can't create a metric");
}


async fn index(method: Method, req: Request) -> impl IntoResponse {
  let start: Instant = Instant::now();
  HTTP_REQUESTS_TOTAL.with_label_values(&[method.to_string(), req.uri().to_string()]).inc();
  let duration: f64 = start.elapsed().as_secs_f64();
  HTTP_RESPONSE_TIME_SECONDS
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


#[tokio::main]
async fn main() {
  let rt: AxumRouter<_> = AxumRouter::new()
    .route("/", get(index))
    .route("/metrics", get(metrics));
  let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, rt).await.unwrap();
}