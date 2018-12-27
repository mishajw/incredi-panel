//! Utility functions

use std::thread;

use crate::error::*;

use byteorder::{BigEndian, WriteBytesExt};
use error_chain::ChainedError;
use sfml::graphics::Color;

/// Start a thread, printing out any errors returned
pub fn start_thread<F: Send + 'static, T: Send + 'static>(
    callback: F,
) -> thread::JoinHandle<Result<T>>
where F: FnOnce() -> Result<T> {
    thread::spawn(|| {
        let result = callback();
        if let Err(ref err) = &result {
            error!("Error in thread: {}", err.display_chain());
        }
        result
    })
}

/// Create a SFML color from a hex string
pub fn hex_color(hex: &str) -> Result<Color> {
    let integer =
        u32::from_str_radix(hex, 16).chain_err(|| "Failed to decode hex")?;
    let mut bytes: Vec<u8> = vec![];
    bytes
        .write_u32::<BigEndian>(integer)
        .chain_err(|| "Failed to split u32 into bytes")?;

    if hex.len() == 6 {
        Ok(Color::rgb(bytes[1], bytes[2], bytes[3]))
    } else if hex.len() == 8 {
        Ok(Color::rgba(bytes[0], bytes[1], bytes[2], bytes[3]))
    } else {
        bail!(ErrorKind::ConfigError(
            "Hex colors must have six or eight characters".into()
        ));
    }
}
