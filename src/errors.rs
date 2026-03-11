use rmcp::ErrorData as McpError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wezterm cli error: {0}")]
    Cli(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        tracing::error!("{err}");
        Self::internal_error(err.to_string(), None)
    }
}
