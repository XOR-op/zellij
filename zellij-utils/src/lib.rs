pub mod cli;
pub mod consts;
pub mod data;
pub mod envs;
pub mod errors;
pub mod home;
pub mod input;
pub mod kdl;
pub mod pane_size;
pub mod plugin_api;
pub mod position;
pub mod session_serialization;
pub mod setup;
pub mod shared;

// The following modules can't be used when targeting wasm
#[cfg(not(target_family = "wasm"))]
pub mod channels; // Requires async_std
#[cfg(not(target_family = "wasm"))]
pub mod common_path;
#[cfg(not(target_family = "wasm"))]
pub mod downloader; // Requires async_std
#[cfg(not(target_family = "wasm"))]
pub mod ipc; // Requires interprocess
#[cfg(not(target_family = "wasm"))]
pub mod logging; // Requires log4rs

// TODO(hartan): Remove this re-export for the next minor release.
pub use ::prost;
