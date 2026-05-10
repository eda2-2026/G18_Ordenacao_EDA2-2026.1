use crate::core::file_metadata::FileMetadata;
use super::Sorter;
use super::common::{compare_files, SortCriteria};
use std::cmp::Ordering;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> usize {
        let mut comparisons = 0;
        let n = files.len();
        for i in 0..n {
            for j in 0..n.saturating_sub(1 + i) {
                comparisons += 1;
                if compare_files(&files[j], &files[j + 1], criteria) == Ordering::Greater {
                    files.swap(j, j + 1);
                }
            }
        }
        comparisons
    }
}
