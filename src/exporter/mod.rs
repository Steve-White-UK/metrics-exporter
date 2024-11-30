use std::future::Future;

use std::pin::Pin;
use std::time::Duration;

use hyper::Uri;

/// Error types possible from an exporter
#[derive(Debug)]
pub enum ExporterError {
    PushGateway(()),
}
/// Convenience type for Future implementing an exporter.
pub type ExporterFuture = Pin<Box<dyn Future<Output = Result<(), ExporterError>> + Send + 'static>>;

#[derive(Clone, Debug)]
enum ExporterConfig {

    // Run a push gateway task sending to the given `endpoint` after `interval` time has elapsed,
    // infinitely.
    PushGateway {
        endpoint: Uri,
        interval: Duration,
        username: Option<String>,
        password: Option<String>,
    },

    #[allow(dead_code)]
    Unconfigured,
}

impl ExporterConfig {
    #[allow(dead_code)]
    fn as_type_str(&self) -> &'static str {
        match self {
            Self::PushGateway { .. } => "push-gateway",
            Self::Unconfigured => "unconfigured,",
        }
    }
}



mod push_gateway;

pub(crate) mod builder;
