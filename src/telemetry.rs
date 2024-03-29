/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber<'a>(
    name: &'a str,
    env_filter: &'a str,
    sink: impl tracing_subscriber::fmt::MakeWriter + Send + Sync + 'static,
) -> impl tracing::Subscriber + Send + Sync {
    use tracing_subscriber::layer::SubscriberExt;

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));
    let formatting_layer =
        tracing_bunyan_formatter::BunyanFormattingLayer::new(name.into(), sink);

    tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("failed to set logger");
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set subscriber");
}
