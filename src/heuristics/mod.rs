use crate::core::{Heuristic, LangData, MatchingState};

mod rust;
mod unity;

pub const ALL_HEURISTICS: [&dyn Heuristic; 2] = [&rust::INSTANCE, &unity::INSTANCE];

#[macro_export]
macro_rules! heuristic {
    ($name:ident,$icon:expr,$short:expr,$state:ident,$expression:expr) => {
        use super::*;

        #[derive(Default)]
        pub struct $name;

        pub const INSTANCE: $name = $name;

        impl Heuristic for $name {
            fn info(&self) -> LangData {
                LangData::new(stringify!($name), $icon, $short)
            }

            fn check_for_matches(&self, $state: &mut MatchingState) {
                $expression
            }
        }
    };
}
