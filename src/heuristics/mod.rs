use crate::core::{ColorIndexed, Heuristic, Lang, MatchingState};

mod rust;
mod unity;

/// A list of all heuristics implemented by default in this crate.
pub const ALL_HEURISTICS: [&dyn Heuristic; 2] = [&rust::INSTANCE, &unity::INSTANCE];

#[macro_export]
macro_rules! heuristic {
    ($name:ident, $icon:literal, $short:literal, $color:expr, $state:ident, $expression:expr) => {
        use super::*;

        #[derive(Default)]
        pub struct $name;

        pub const INSTANCE: $name = $name;

        impl Heuristic for $name {
            fn info(&self) -> &'static Lang {
                const LANG: Lang = Lang {
                    name: stringify!($name),
                    icon: $icon,
                    short: $short,
                    color: $color,
                };
                &LANG
            }

            fn check_for_matches(&self, $state: &mut MatchingState) {
                $expression
            }
        }
    };
}
