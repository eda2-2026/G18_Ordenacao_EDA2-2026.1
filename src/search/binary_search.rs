use crate::core::file_metadata::FileMetadata;
use super::Searcher;

pub struct BinarySearch;

impl Searcher for BinarySearch {
    /// Busca binária por prefixo (case-insensitive).
    /// Requer que `files` esteja ordenado por nome antes de chamar.
    fn search(files: &[FileMetadata], query: &str) -> (Vec<usize>, usize) {
        if files.is_empty() || query.is_empty() {
            return (vec![], 0);
        }

        let query_lower = query.to_lowercase();
        let mut comparisons = 0usize;
        let mut lo = 0isize;
        let mut hi = files.len() as isize - 1;
        let mut first_match: Option<usize> = None;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let name = files[mid as usize].name.to_lowercase();
            comparisons += 1;

            if name.starts_with(&query_lower) {
                first_match = Some(mid as usize);
                hi = mid - 1;
            } else if name < query_lower {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        let mut indices = vec![];
        if let Some(start) = first_match {
            let mut i = start;
            while i < files.len() {
                comparisons += 1;
                if files[i].name.to_lowercase().starts_with(&query_lower) {
                    indices.push(i);
                    i += 1;
                } else {
                    break;
                }
            }
        }

        (indices, comparisons)
    }
}
