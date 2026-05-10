use crate::core::file_metadata::FileMetadata;
use crate::search::common::Searcher;

pub struct LinearSearch;

impl Searcher for LinearSearch {
    fn search(files: &[FileMetadata], query: &str) -> (Vec<usize>, usize) {
        let mut matches = Vec::new();
        let mut comparisons = 0;
        let query_lower = query.to_lowercase();

        for (i, file) in files.iter().enumerate() {
            comparisons += 1;
            if file.name.to_lowercase().contains(&query_lower) || file.extension().to_lowercase() == query_lower {
                matches.push(i);
            }
        }

        (matches, comparisons)
    }
}
