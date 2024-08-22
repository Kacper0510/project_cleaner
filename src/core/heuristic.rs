use super::MatchingState;
use std::fmt;

/// Trait for implementing heuristics to match directories and files for deletion.
pub trait Heuristic {
    /// Returns information about the heuristic.
    /// 
    /// This information is used to display the heuristic in the UI.
    fn info(&self) -> LangData;
    /// Find matches in a directory and adds results to the state.
    /// 
    /// All actions in this method should be performed on the `state` parameter.
    fn check_for_matches(&self, state: &mut MatchingState);
}

impl fmt::Debug for dyn Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.info())
    }
}

impl fmt::Display for dyn Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}

/// Data structure representing a programming language or other reason for a match.
#[derive(Debug, Clone)]
pub struct LangData {
    /// Name of the language or heuristic.
    name: &'static str,
    /// Icon representing the language or heuristic.
    /// 
    /// This icon should be an emoji or [a nerd font symbol](https://www.nerdfonts.com/).
    icon: &'static str,
    /// Short name/abbreviation of the language or heuristic, used when icons are not supported.
    short: &'static str,
    /// Comment for this instance of [`LangData`], present only when querying specific match information.
    comment: Option<String>,
}

impl fmt::Display for LangData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(comment) = &self.comment {
            write!(f, " - {}", comment)
        } else {
            Ok(())
        }
    }
}

impl LangData {
    /// Creates a new instance of [`LangData`] with the specified parameters.
    #[inline]
    pub const fn new(name: &'static str, icon: &'static str, short: &'static str) -> Self {
        Self {
            name,
            icon,
            short,
            comment: None,
        }
    }

    /// Returns the name of the language or heuristic.
    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the icon representing the language or heuristic.
    /// 
    /// This icon should be an emoji or [a nerd font symbol](https://www.nerdfonts.com/).
    #[inline]
    pub fn icon(&self) -> &str {
        self.icon
    }

    /// Returns the abbreviation of the language or heuristic, used when icons are not supported.
    #[inline]
    pub fn short(&self) -> &str {
        self.short
    }

    /// Returns a comment for this instance of [`LangData`].
    /// 
    /// `None` when querying general heuristic information.
    /// `Some` when querying specific match information.
    #[inline]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Sets a comment for this instance of [`LangData`].
    #[inline]
    pub(super) fn with_comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}