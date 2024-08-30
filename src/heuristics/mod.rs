use crate::core::Heuristic;

mod git;
mod hidden;
mod js;
mod python;
mod rust;
mod system;
mod unity;

/// A list of all heuristics implemented by default in this crate.
///
/// According to [the Compiler Explorer](https://godbolt.org/),
/// a slice generates even less assembly code than an array (weird, isn't it?).
pub const ALL_HEURISTICS: &[&dyn Heuristic] = &[
    &hidden::INSTANCE,
    &system::INSTANCE,
    &rust::INSTANCE,
    &unity::INSTANCE,
    &js::INSTANCE,
    &python::INSTANCE,
    &git::INSTANCE,
];

/// Simplified heuristic declaration.
///
/// Parameters in order:
/// - `name` - heuristic name, also used as a generated struct indentifier,
/// - `icon` - Nerd Font icon (see [`Lang::icon`]),
/// - `short` - heuristic name abbreviation (see [`Lang::short`]),
/// - `color` - [`IconColor`] instance,
/// - `state` - parameter name for [`Heuristic::check_for_matches()`],
/// - `expression` - heuristic body with state in scope.
#[macro_export]
macro_rules! heuristic {
    ($name:ident, $icon:expr, $short:literal, $color:expr, $state:ident, $expression:expr) => {
        use $crate::core::{Heuristic, IconColor, Lang, MatchingState};

        #[derive(Default)]
        pub struct $name;

        #[allow(dead_code)]
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
