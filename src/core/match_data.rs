use super::CommentedLang;
use std::{iter::Sum, ops::Add, path::{Path, PathBuf}};

/// Data structure representing a match found by one or more heuristics.
#[derive(Debug, Clone)]
pub struct MatchData {
    /// Path of the matched file.
    pub path: PathBuf,
    /// Whether the match originated from a dangerous (possibly system-managed) path.
    /// 
    /// Always false if the [corresponding `Scanner` field](super::Scanner::dangerous) is turned off. 
    pub dangerous: bool,
    /// Path of the directory which was being processed while the match was found.
    ///
    /// Used to group multiple results in a meaningful way.
    pub(super) group: PathBuf,
    /// Additional data about the match, queried with implemented methods.
    pub(super) params: MatchParameters,
}

impl MatchData {
    /// Returns path of the directory which was being processed while the match was found
    /// or a custom override added by one or more heuristics.
    ///
    /// Used to group multiple results in a meaningful way.
    #[inline]
    pub fn group(&self) -> &Path {
        if let GroupOverride::Override(custom) = &self.params.group_override {
            custom
        } else {
            &self.group
        }
    }

    /// Returns the final sum of weights of the match. Guaranteed to be positive.
    #[inline]
    pub fn weight(&self) -> u32 {
        self.params.weight as u32
    }

    /// Returns the reasons for the match, which are mostly different programming languages.
    #[inline]
    pub fn languages(&self) -> &[CommentedLang] {
        &self.params.languages
    }
}

/// Main match parameters that a heuristic sets, returned from [`add_match()`](super::MatchingState::add_match()).
/// Accumulated version accessible via getters in [`MatchData`].
#[derive(Debug, Clone, Default)]
pub struct MatchParameters {
    /// Weight of the match.
    pub(super) weight: i32,
    /// Reasons for the match, mostly different programming languages.
    pub(super) languages: Vec<CommentedLang>,
    /// Whether the default group path in [`MatchData`] should be overridden.
    pub(super) group_override: GroupOverride,
}

impl MatchParameters {
    /// Default weight for a match if not specified via [`Self::weight()`].
    pub const DEFAULT_WEIGHT: i32 = 1000;

    pub(super) fn new(lang: CommentedLang) -> Self {
        Self {
            languages: vec![lang],
            weight: Self::DEFAULT_WEIGHT,
            ..Self::default()
        }
    }

    /// Sets custom weight for the newly added match. May be negative to indicate dangerous paths/files.
    #[inline]
    pub fn weight(&mut self, weight: i32) -> &mut Self {
        self.weight = weight;
        self
    }

    /// Suggests custom group for the newly added match.
    /// 
    /// It may not be considered in the final result if a custom group conflict occurs.
    #[inline]
    pub fn custom_group(&mut self, group: PathBuf) -> &mut Self {
        self.group_override = GroupOverride::Override(group);
        self
    }
}

impl Add for MatchParameters {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.weight += rhs.weight;
        self.languages.extend(rhs.languages);
        self.group_override = self.group_override + rhs.group_override;
        self
    }
}

impl Sum for MatchParameters {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

/// Represents different verdicts on overriding default group path.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) enum GroupOverride {
    /// No override was specified in any heuristic.
    #[default]
    None,
    /// A singular or multiple non-conflicting overrides were specified.
    Override(PathBuf),
    /// Override conflict was detected, defaulting to parent path for group.
    Conflict,
}

impl Add for GroupOverride {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (GroupOverride::None, other) | (other, GroupOverride::None) => other,
            (GroupOverride::Override(p1), GroupOverride::Override(p2)) if p1 == p2 => GroupOverride::Override(p1),
            _ => GroupOverride::Conflict,
        }
    }
}
