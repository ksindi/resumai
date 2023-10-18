use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

/// Setup logging
pub fn setup_logging() {
    FmtSubscriber::builder()
        .json()
        .flatten_event(true)
        .with_ansi(false)
        .with_current_span(false)
        .with_span_list(true)
        .without_time()
        .finish()
        .init();
}
