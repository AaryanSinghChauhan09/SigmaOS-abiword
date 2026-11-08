//! Command-line options for abiword.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Create new document
    New,
    /// Open document
    Open,
    /// Save document
    Save,
    /// Print document
    Print,
    /// Export document
    Export,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Document path
    pub document_path: String,
    /// Output path
    pub output_path: String,
    /// Document format
    pub format: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::New,
            document_path: String::new(),
            output_path: String::new(),
            format: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
