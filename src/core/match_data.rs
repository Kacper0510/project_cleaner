use super::LangData;
use std::{
    iter::Sum,
    ops::{Add, AddAssign},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct MatchData {
    pub path: PathBuf,
    pub group: PathBuf,
    pub(super) other_data: MatchDataBuilder,
}

impl MatchData {
    pub fn weight(&self) -> u32 {
        self.other_data.weight as u32
    }

    pub fn languages(&self) -> &[LangData] {
        &self.other_data.reasons
    }

    pub fn hidden(&self) -> bool {
        self.other_data.hidden
    }
}

#[derive(Debug, Clone, Default)]
pub struct MatchDataBuilder {
    pub(super) weight: i32,
    pub(super) reasons: Vec<LangData>,
    pub(super) hidden: bool,
}

impl MatchDataBuilder {
    pub const DEFAULT_WEIGHT: i32 = 1000;

    pub(super) fn new(lang: LangData) -> Self {
        Self {
            reasons: vec![lang],
            weight: Self::DEFAULT_WEIGHT,
            ..Self::default()
        }
    }

    pub fn weight(&mut self, weight: i32) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }
}

impl AddAssign for MatchDataBuilder {
    fn add_assign(&mut self, rhs: Self) {
        self.weight += rhs.weight;
        self.reasons.extend(rhs.reasons);
        self.hidden |= rhs.hidden;
    }
}

impl Add for MatchDataBuilder {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Sum for MatchDataBuilder {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}
