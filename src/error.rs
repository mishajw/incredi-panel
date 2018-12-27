//! Error types used across project

#[allow(missing_docs)]
error_chain! {
    errors {
        /// Error running a command
        CommandError(message: String) {
            display("Error executing command: {}", message)
        }
        /// Error in configuration
        ConfigError(message: String) {
            display("Error with configuration: {}", message)
        }
    }
}
