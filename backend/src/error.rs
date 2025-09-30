//! Error types for the document path management system

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    
    #[error("Invalid path: must be absolute path")]
    RelativePathNotAllowed,
    
    #[error("Invalid type code: {0}")]
    InvalidTypeCode(String),
    
    #[error("Department not found: {0}")]
    DepartmentNotFound(char),
    
    #[error("Section not found: {0}")]
    SectionNotFound(char),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Business task not found: {0}")]
    BusinessTaskNotFound(String),
    
    #[error("User not authorized for document type")]
    UnauthorizedDocumentType,
    
    #[error("Duplicate document number: {0}")]
    DuplicateDocumentNumber(String),
    
    #[error("Concurrent modification detected")]
    ConcurrentModification,
    
    #[error("Invalid rule component: {0}")]
    InvalidRuleComponent(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
