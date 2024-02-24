#![warn(missing_docs)]
//! rsgex, a convenient regex tool.
//!
use std::fmt::Display;
/// Convenient re-exports for rsgex.
pub mod prelude;
/// Contains methods to sanitize and contain user's regex pattern.
pub struct RegexContainer {
    pattern: String,
}
impl RegexContainer {
    /// Creates new RegexContainer.
    pub fn new(pattern: String) -> RegexContainer {
        RegexContainer { pattern }
    }
    /// Sanitizes user's regex.
    pub fn sanitize(mut self) -> Self {
        self.pattern = self.pattern.trim_end_matches("\n").to_string();
        self
    }
}
impl Display for RegexContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{}", self.pattern)
    }
}
/// Contains user's haystack.
pub struct HayContainer {
    hay: String,
}
impl HayContainer {
    /// Creates a new haystack
    pub fn new(hay: String) -> HayContainer {
        HayContainer { hay }
    }
    /// Returns user's haystack as a string slice.
    pub fn get(&self) -> &str {
        self.hay.as_str()
    }
}
/// A trait that defines valid process output.
pub trait CreateOutput {
    /// Creates a new valid output.
    fn new_output(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Pattern => OutputKind::Pattern(UserPattern { pattern: s }),
            InputKind::Hay => OutputKind::Hay(UserHay { hay: s }),
        }
    }
}
/// Contains the pattern input from the terminal.
pub struct UserPattern {
    pattern: String,
}
impl UserPattern {
    /// Creates a new UserPattern.
    pub fn new() -> UserPattern {
        UserPattern {
            pattern: String::new(),
        }
    }
}
impl CreateOutput for UserPattern {
    fn new_output(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Pattern => OutputKind::Pattern(UserPattern { pattern: s }),
            _ => OutputKind::Default,
        }
    }
}
impl Display for UserPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{}", self.pattern)
    }
}
/// Contains the hay input from the terminal.
pub struct UserHay {
    hay: String,
}
impl UserHay {
    /// Creates a new haystack from the terminal.
    pub fn new() -> UserHay {
        UserHay { hay: String::new() }
    }
}
impl CreateOutput for UserHay {
    fn new_output(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Hay => OutputKind::Hay(UserHay { hay: s }),
            _ => OutputKind::Default,
        }
    }
}
impl Display for UserHay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hay)
    }
}
/// Enumerations for valid user inputs.
pub enum InputKind {
    /// User's regex pattern.
    Pattern,
    /// User's haystack.
    Hay,
}
/// Enumerations for valid process outputs.
pub enum OutputKind {
    /// Outputs a regex pattern as a UserPattern.
    Pattern(UserPattern),
    /// Outputs a haystack as a UserHay.
    Hay(UserHay),
    /// Default output when it is neither a UserHay nor a UserPattern.
    Default,
}
