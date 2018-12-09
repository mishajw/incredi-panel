use std::thread;

use error::*;

use error_chain::ChainedError;

/// Start a thread, printing out any errors returned
pub fn start_thread<F: Send + 'static, T: Send + 'static>(
    callback: F,
) -> thread::JoinHandle<Result<T>>
where F: FnOnce() -> Result<T> {
    thread::spawn(|| {
        let result = callback();
        if let Err(ref err) = &result {
            eprintln!("Error in thread: {}", err.display_chain());
        }
        result
    })
}
