use crate::core::file_metadata::FileMetadata;
use crate::sorting::{compare_files, SortCriteria, Sorter};

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        let len = files.len();
        if len > 1 {
            quick_sort_recursive(files, 0, (len - 1) as isize, criteria)
        } else {
            (0, 0)
        }
    }
}

fn quick_sort_recursive(files: &mut [FileMetadata], low: isize, high: isize, criteria: SortCriteria) -> (usize, usize) {
    let mut comps = 0;
    let mut swaps = 0;
    if low < high {
        let (p, c, s) = partition(files, low, high, criteria);
        comps += c;
        swaps += s;
        
        let (c1, s1) = quick_sort_recursive(files, low, p - 1, criteria);
        let (c2, s2) = quick_sort_recursive(files, p + 1, high, criteria);
        
        comps += c1 + c2;
        swaps += s1 + s2;
    }
    (comps, swaps)
}

fn partition(files: &mut [FileMetadata], low: isize, high: isize, criteria: SortCriteria) -> (isize, usize, usize) {
    let mut comps = 0;
    let mut swaps = 0;
    let pivot_idx = high as usize;
    let mut i = low - 1;

    for j in low..high {
        comps += 1;
        if compare_files(&files[j as usize], &files[pivot_idx], criteria) != std::cmp::Ordering::Greater {
            i += 1;
            files.swap(i as usize, j as usize);
            swaps += 1;
        }
    }
    files.swap((i + 1) as usize, high as usize);
    swaps += 1;
    (i + 1, comps, swaps)
}
