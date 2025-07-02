use std::fmt;
use std::io;

#[derive(Debug)]
pub enum JkmsError {
    Prompt(PromptError),
    Storage(StorageError),
    External(ExternalError),
    Validation(ValidationError),
}

#[derive(Debug)]
pub enum PromptError {
    NotFound(String),
    AlreadyExists(String),
    InvalidFormat(String),
}

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    ParseError(String),
    InvalidPath(String),
}

#[derive(Debug)]
pub enum ExternalError {
    ClipboardError(String),
    EditorError(String),
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidInput(&'static str, String),
    MissingRequired(String),
}

impl fmt::Display for JkmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JkmsError::Prompt(e) => write!(f, "{}", e),
            JkmsError::Storage(e) => write!(f, "{}", e),
            JkmsError::External(e) => write!(f, "{}", e),
            JkmsError::Validation(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromptError::NotFound(name) => write!(f, "Prompt not found: {}", name),
            PromptError::AlreadyExists(name) => write!(f, "Prompt already exists: {}", name),
            PromptError::InvalidFormat(msg) => write!(f, "Invalid prompt format: {}", msg),
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "IO error: {}", e),
            StorageError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            StorageError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
        }
    }
}

impl fmt::Display for ExternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExternalError::ClipboardError(msg) => write!(f, "Clipboard error: {}", msg),
            ExternalError::EditorError(msg) => write!(f, "Editor error: {}", msg),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidInput(field, msg) => {
                write!(f, "Invalid input for '{}': {}", field, msg)
            }
            ValidationError::MissingRequired(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for JkmsError {}
impl std::error::Error for PromptError {}
impl std::error::Error for StorageError {}
impl std::error::Error for ExternalError {}
impl std::error::Error for ValidationError {}

impl From<io::Error> for JkmsError {
    fn from(error: io::Error) -> Self {
        JkmsError::Storage(StorageError::Io(error))
    }
}

impl From<anyhow::Error> for JkmsError {
    fn from(error: anyhow::Error) -> Self {
        // Try to downcast to io::Error first
        if let Some(io_err) = error.downcast_ref::<io::Error>() {
            return JkmsError::Storage(StorageError::Io(io::Error::new(io_err.kind(), error.to_string())));
        }
        
        // Otherwise, treat as a generic storage error
        JkmsError::Storage(StorageError::ParseError(error.to_string()))
    }
}

impl JkmsError {
    pub fn user_message(&self) -> String {
        match self {
            JkmsError::Prompt(PromptError::NotFound(name)) => {
                format!(
                    "Could not find prompt '{}'. Try:\n  - Check the prompt name\n  - Run 'jkms list' to see available prompts\n  - Create it with 'jkms create {}'",
                    name, name
                )
            }
            JkmsError::Prompt(PromptError::AlreadyExists(name)) => {
                format!(
                    "Prompt '{}' already exists. Try:\n  - Use a different name\n  - Edit the existing prompt with 'jkms edit {}'",
                    name, name
                )
            }
            JkmsError::Storage(StorageError::Io(e)) if e.kind() == io::ErrorKind::PermissionDenied => {
                "Permission denied. Check file permissions or run with appropriate privileges.".to_string()
            }
            _ => self.to_string(),
        }
    }
    
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            JkmsError::Prompt(PromptError::NotFound(_)) |
            JkmsError::Prompt(PromptError::AlreadyExists(_)) |
            JkmsError::Validation(_)
        )
    }
}

// Result type alias for convenience
pub type Result<T> = std::result::Result<T, JkmsError>;