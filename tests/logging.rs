#[cfg(test)]
mod logging_tests {
    use tracing::info;
    use utils::logging;

    #[test]
    fn test_init() {
        logging::setup("RUST_LOG=info,tower_http=debug".to_string()).expect("unable to create logger");

        assert!((|| {
            info!("it works");
            true
        })());
    }
}
