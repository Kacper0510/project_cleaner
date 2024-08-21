use super::MatchingState;
use std::fmt;

pub trait Heuristic {
    fn info(&self) -> LangData;
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

#[derive(Debug, Clone)]
pub struct LangData {
    pub name: &'static str,
    pub icon: &'static str,
    pub short: &'static str,
    pub comment: Option<String>,
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
    pub const fn new(name: &'static str, icon: &'static str, short: &'static str) -> Self {
        Self {
            name,
            icon,
            short,
            comment: None,
        }
    }

    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}