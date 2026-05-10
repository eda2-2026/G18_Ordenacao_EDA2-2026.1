use crate::core::file_metadata::FileMetadata;
use super::Sorter;
use super::common::{compare_files, SortCriteria};
use std::cmp::Ordering;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        let mut comparisons = 0;
        let mut swaps = 0;
        let n = files.len();
        for i in 1..n {
            let mut j = i;
            while j > 0 {
                comparisons += 1;
                if compare_files(&files[j - 1], &files[j], criteria) == Ordering::Greater {
                    files.swap(j - 1, j);
                    swaps += 1;
                    j -= 1;
                } else {
                    break;
                }
            }
        }
        (comparisons, swaps)
    }
}
