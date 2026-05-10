use crate::core::file_metadata::FileMetadata;
use super::Sorter;
use super::common::{compare_files, SortCriteria};
use std::cmp::Ordering;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        let mut comparisons = 0;
        let mut swaps = 0;
        let n = files.len();
        for i in 0..n {
            let mut min = i;
            for j in i + 1..n {
                comparisons += 1;
                if compare_files(&files[j], &files[min], criteria) == Ordering::Less {
                    min = j;
                }
            }
            if min != i {
                files.swap(i, min);
                swaps += 1;
            }
        }
        (comparisons, swaps)
    }
}
