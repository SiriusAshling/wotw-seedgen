#[cfg(feature = "loc_data")]
mod loc_data;
#[cfg(feature = "loc_data")]
pub use loc_data::*;
#[cfg(feature = "state_data")]
mod state_data;
#[cfg(feature = "state_data")]
pub use state_data::*;
#[cfg(feature = "uber_state_data")]
mod uber_state_data;
#[cfg(feature = "uber_state_data")]
pub use uber_state_data::*;

/// Representation of a source file with the necessary information to display useful error messages.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Source {
    /// An identifier to be used in error messages that should allow the reader to determine which file the error originated from.
    ///
    /// This might be the file path relative to the workspace root, or just the filename.
    pub id: String,
    /// The contents of the file, which will be referenced for better error messages.
    ///
    /// This should be the same contents you were parsing, otherwise error messages will reference arbitrary spans in your source and possibly panic.
    pub content: String, // TODO maybe use &str?
}
impl Source {
    pub fn new(id: String, content: String) -> Self {
        Self { id, content }
    }
}

pub trait SnippetAccess {
    fn read_snippet(&self, identifier: &str) -> std::result::Result<Source, String>;
}
