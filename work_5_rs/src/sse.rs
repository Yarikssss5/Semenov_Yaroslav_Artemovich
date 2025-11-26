use axum::response::sse::{Event, KeepAlive, Sse};
use axum_extra::{TypedHeader, headers::{self}};
use futures_util::stream::{self, Stream};
use tokio::time::Instant;
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

use crate::{SSEGuard, metrics};


pub(crate) async fn sse_handler(TypedHeader(user_agent): TypedHeader<headers::UserAgent>
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    let start = Instant::now();
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    let duration: f64 = start.elapsed().as_secs_f64();
    let _guard: SSEGuard = SSEGuard::new();
    
    metrics::HTTP_RESPONSE_TIME_SECONDS
        .with_label_values(&["GET", "/sse"])
        .observe(duration);
    
    metrics::HTTP_REQUESTS_TOTAL.with_label_values(&["GET", "/sse"]).inc();

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}