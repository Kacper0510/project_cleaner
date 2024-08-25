use crate::heuristic;
use ignore::{gitignore::Gitignore, Match};

enum GitMatchWeight {
    /// File was not matched, maybe the user wants to leave it as is?
    NotMatched = -1_000,
    /// File was ignored, so it probably can be removed.
    Ignored = 500,
    /// File was explicitly whitelisted, we are not touching it.
    Whitelisted = -10_000,
}

impl GitMatchWeight {
    const fn comment(&self) -> &'static str {
        match self {
            GitMatchWeight::NotMatched => "File was not included in any .gitignore files that were found.",
            GitMatchWeight::Ignored => "File was included in one of .gitignore files.",
            GitMatchWeight::Whitelisted => "File was explicitly whitelisted in one of .gitignore files.",
        }
    }
}

impl<T> From<Match<T>> for GitMatchWeight {
    fn from(value: Match<T>) -> Self {
        match value {
            Match::None => GitMatchWeight::NotMatched,
            Match::Ignore(_) => GitMatchWeight::Ignored,
            Match::Whitelist(_) => GitMatchWeight::Whitelisted,
        }
    }
}

heuristic!(Git, "", "git", ColorIndexed(202), state, {
    if let Some(gitignore) = state.has_file(".gitignore") {
        state.inherited_files().push(gitignore);
    }

    if state.inherited_files().is_empty() {
        return;
    }

    let matchers: Vec<_> = state.inherited_files().iter().rev().map(|ignore| Gitignore::new(ignore).0).collect();
    let matches: Vec<_> = state
        .get_all_contents()
        .filter(|(path, _)| path.file_name().is_some())
        .map(|(path, ptype)| {
            let final_verdict =
                matchers.iter().map(|m| m.matched(&path, ptype.is_dir())).fold(Match::None, |a, b| a.or(b));
            let weight: GitMatchWeight = final_verdict.into();
            (path.file_name().unwrap().to_owned(), weight)
        })
        .collect();
    for (name, weight) in matches {
        state.add_match(&name, weight.comment()).weight(weight as i32);
    }
});
