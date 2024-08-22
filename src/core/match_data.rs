use super::LangData;
use std::{
    iter::Sum,
    ops::{Add, AddAssign},
    path::PathBuf,
};

/// Data structure representing a match found by one or more heuristics.
#[derive(Debug, Clone)]
pub struct MatchData {
    /// Path of the matched file.
    pub path: PathBuf,
    /// Path of the directory which was being processed while the match was found.
    ///
    /// Used to group multiple results in a meaningful way.
    pub group: PathBuf,
    /// Additional data about the match, queried with implemented methods.
    pub(super) params: MatchParameters,
}

impl MatchData {
    /// Returns the final sum of weights of the match. Guaranteed to be positive.
    #[inline]
    pub fn weight(&self) -> u32 {
        self.params.weight as u32
    }

    /// Returns the reasons for the match, which are mostly different programming languages.
    #[inline]
    pub fn languages(&self) -> &[LangData] {
        &self.params.languages
    }

    /// Returns whether the match should be hidden/excluded while displaying/deleting files and directories.
    #[inline]
    pub fn hidden(&self) -> bool {
        self.params.hidden
    }
}

/// Main match parameters that a heuristic sets, returned from [`add_match()`](super::MatchingState::add_match()).
/// Accumulated version accessible via getters in [`MatchData`].
#[derive(Debug, Clone, Default)]
pub struct MatchParameters {
    /// Weight of the match.
    pub(super) weight: i32,
    /// Reasons for the match, mostly different programming languages.
    pub(super) languages: Vec<LangData>,
    /// Whether the match should be hidden/excluded while displaying/deleting files and directories.
    pub(super) hidden: bool,
}

impl MatchParameters {
    /// Default weight for a match if not specified via [`Self::weight()`].
    pub const DEFAULT_WEIGHT: i32 = 1000;

    pub(super) fn new(lang: LangData) -> Self {
        Self {
            languages: vec![lang],
            weight: Self::DEFAULT_WEIGHT,
            ..Self::default()
        }
    }

    /// Sets custom weight for the newly added match. May be negative.
    #[inline]
    pub fn weight(&mut self, weight: i32) -> &mut Self {
        self.weight = weight;
        self
    }

    /// Sets the `hidden` flag for the newly added match.
    #[inline]
    pub fn hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }
}

impl AddAssign for MatchParameters {
    fn add_assign(&mut self, rhs: Self) {
        self.weight += rhs.weight;
        self.languages.extend(rhs.languages);
        self.hidden |= rhs.hidden;
    }
}

impl Add for MatchParameters {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Sum for MatchParameters {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}
