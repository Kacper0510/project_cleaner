use super::{Lang, MatchingState};
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
