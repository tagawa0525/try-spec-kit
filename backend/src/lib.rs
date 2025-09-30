//! Document Path Management Database
//! 
//! A system for managing document file paths with customizable numbering rules.

pub mod models;
pub mod storage;
pub mod services;
pub mod api;
pub mod error;
pub mod validation;

pub use error::{Error, Result};
