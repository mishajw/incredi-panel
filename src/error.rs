#[allow(missing_docs)]
error_chain! {
    errors {
        CommandError(message: String) {
            display("Error executing command: {}", message)
        }
        ConfigError(message: String) {
            display("Error with configuration: {}", message)
        }
    }
}
