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

/// Change an SDL result to have the local error type
/// TODO: Remove this function
pub fn wrap_sdl_result<T>(result: std::result::Result<T, String>) -> Result<T> {
    result.map_err::<Error, _>(|e| ErrorKind::SdlError(e).into())
}
