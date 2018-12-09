#[allow(missing_docs)]
error_chain! {
    errors {
        CommandError(message: String) {
            display("Error executing command: {}", message)
        }
        SdlError(message: String) {
            display("Error with SDL2: {}", message)
        }
    }
}
