use crate::core::file_metadata::FileMetadata;
use std::path::Path;
use walkdir::WalkDir;

pub struct RecursiveSearch;

impl RecursiveSearch {
    pub fn search(root: &Path, query: &str) -> (Vec<FileMetadata>, Vec<usize>, usize) {
        let mut all_files = Vec::new();
        let mut matches = Vec::new();
        let mut comparisons = 0;
        let query_lower = query.to_lowercase();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if FileMetadata::is_hidden(&entry) { continue; }
            let metadata = FileMetadata::from_entry(&entry);
            all_files.push(metadata);
        }

        for (i, file) in all_files.iter().enumerate() {
            comparisons += 1;
            if file.name.to_lowercase().contains(&query_lower) || file.extension().to_lowercase() == query_lower {
                matches.push(i);
            }
        }

        (all_files, matches, comparisons)
    }
}
