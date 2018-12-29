//! Library for incredi panel

#![warn(missing_docs)]

extern crate quick_xml;
extern crate yaml_rust;
#[macro_use]
extern crate error_chain;
extern crate sfml;
#[macro_use]
extern crate log;
extern crate byteorder;
extern crate nix;

#[macro_use]
pub mod config;
mod anchor;
mod dock;
pub mod error;
pub mod ipc;
pub mod item;
pub mod util;
pub mod window;
pub use self::anchor::Anchor;
pub use self::dock::dock_window;
