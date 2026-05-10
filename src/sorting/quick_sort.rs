use crate::core::file_metadata::FileMetadata;
use crate::sorting::{compare_files, SortCriteria, Sorter};

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> usize {
        let len = files.len();
        if len > 1 {
            quick_sort_recursive(files, 0, (len - 1) as isize, criteria)
        } else {
            0
        }
    }
}

fn quick_sort_recursive(files: &mut [FileMetadata], low: isize, high: isize, criteria: SortCriteria) -> usize {
    let mut comps = 0;
    if low < high {
        let (p, c) = partition(files, low, high, criteria);
        comps += c;
        comps += quick_sort_recursive(files, low, p - 1, criteria);
        comps += quick_sort_recursive(files, p + 1, high, criteria);
    }
    comps
}

fn partition(files: &mut [FileMetadata], low: isize, high: isize, criteria: SortCriteria) -> (isize, usize) {
    let mut comps = 0;
    let pivot_idx = high as usize;
    let mut i = low - 1;

    for j in low..high {
        comps += 1;
        if compare_files(&files[j as usize], &files[pivot_idx], criteria) != std::cmp::Ordering::Greater {
            i += 1;
            files.swap(i as usize, j as usize);
        }
    }
    files.swap((i + 1) as usize, high as usize);
    (i + 1, comps)
}
