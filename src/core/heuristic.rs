use super::{color::IconColor, MatchingState};
use std::fmt;

/// Trait for implementing heuristics to match directories and files for deletion.
pub trait Heuristic {
    /// Returns information about the heuristic.
    ///
    /// This information is used to display the heuristic in the UI.
    fn info(&self) -> &'static Lang;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lang {
    /// Name of the language or heuristic.
    pub name: &'static str,
    /// Icon representing the language or heuristic.
    ///
    /// This icon should be an emoji or [a nerd font symbol](https://www.nerdfonts.com/).
    pub icon: &'static str,
    /// Short name/abbreviation of the language or heuristic, used when icons are not supported.
    pub short: &'static str,
    /// [ANSI 8-bit color index](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) for the language or heuristic.
    pub color: IconColor,
}

impl PartialOrd for Lang {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lang {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(other.name)
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Extended [`Lang`] data structure with a comment field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentedLang {
    /// Language this struct is based on.
    pub lang: &'static Lang,
    /// Additional comment.
    pub comment: String,
}

impl fmt::Display for CommentedLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.lang.name, self.comment)
    }
}

impl CommentedLang {
    /// Returns the name of the language or heuristic.
    #[inline]
    pub fn name(&self) -> &str {
        self.lang.name
    }

    /// Returns the icon representing the language or heuristic.
    ///
    /// This icon should be an emoji or [a nerd font symbol](https://www.nerdfonts.com/).
    #[inline]
    pub fn icon(&self) -> &str {
        self.lang.icon
    }

    /// Returns the abbreviation of the language or heuristic, used when icons are not supported.
    #[inline]
    pub fn short(&self) -> &str {
        self.lang.short
    }
}
