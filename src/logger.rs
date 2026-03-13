use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
  let format = fmt::format()
    .without_time()
    .with_target(false)
    .with_thread_ids(false)
    .compact();

  let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

  tracing_subscriber::fmt()
    .event_format(format)
    .with_env_filter(filter)
    .init();
}
