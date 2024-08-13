use std::collections::HashMap;

pub mod heuristics;

pub fn walk_directories(root_path: &std::path::Path) -> HashMap<std::path::PathBuf, Vec<heuristics::MatchInfo>> {
    let mut gathered_data = HashMap::new();
    for directory in jwalk::WalkDir::new(root_path).skip_hidden(false).into_iter() {
        if directory.is_err() {
            continue;
        }
        let directory = directory.unwrap();

        for heuristic in heuristics::ALL_HEURISTICS {
            if heuristic.filter(&directory) {
                heuristic.update_matches(directory.path(), &mut gathered_data);
            }
        }
    }
    gathered_data
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]
    fn walk_home_dir() {
        let home = std::env::home_dir().unwrap().join("files").join("coding").join("rust");
        assert!(home.exists() && home.is_dir());
        let data = walk_directories(&home);
        assert!(!data.is_empty());
        println!("{:#?}", data);
    }
}
