use super::*;

#[derive(Default)]
pub struct CargoTarget;

pub const INSTANCE: CargoTarget = CargoTarget;

impl Heuristic for CargoTarget {
    fn filter(&self, entry: &Entry) -> bool {
        entry.file_type().is_dir() && entry.file_name() == "target"
    }

    fn update_matches(&self, path: PathBuf, data: &mut HashMap<PathBuf, Vec<MatchInfo>>) {
        if path.parent().unwrap().join("Cargo.toml").exists() {
            data.entry(path).or_default().push(MatchInfo {
                match_type: "Cargo target".into(),
                weight: 1000,
            });
        }
    }
}
