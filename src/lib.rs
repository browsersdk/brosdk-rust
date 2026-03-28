//! Brosdk SDK - Rust  bindings for the Browser SDK
//!
//! This library provides Rust bindings for interacting with the native Brosdk browser SDK.
//! It loads the native dynamic library at runtime and provides safe wrappers around the C FFI.
//!
//! # Usage
//!
//! ```rust
//! use brosdk_sdk::{load, init, browser_open};
//!
//! // Load the SDK library
//! load(app_handle, "/path/to/brosdk.dll")?;
//!
//! // Initialize with credentials
//! init("user_sig", "/work/dir", 8080)?;
//!
//! // Open a browser
//! browser_open(r#"{"url": "https://example.com"}"#)?;
//! ```

pub mod brosdk;

pub use brosdk::ffi::{
    BrosdkLib, SdkCookiesStorageCb, SdkHandleT, SdkResultCb,
};
pub use brosdk::manager::{
    browser_close, browser_open, init, load, sdk_env_create, sdk_env_page, sdk_info, shutdown, token_update, SdkEvent,
};

/// SDK result type
pub type Result<T> = std::result::Result<T, String>;
