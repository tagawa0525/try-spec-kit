//! Document Path Management Database
//!
//! A system for managing document file paths with customizable numbering rules.

pub mod api;
pub mod error;
pub mod models;
pub mod services;
pub mod storage;
pub mod validation;

pub use error::{Error, Result};
