use crate::core::file_metadata::FileMetadata;
use crate::sorting::{compare_files, SortCriteria, Sorter};

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        let len = files.len();
        if len > 1 {
            let mut temp = files.clone();
            merge_sort_recursive(files, &mut temp, 0, len - 1, criteria)
        } else {
            (0, 0)
        }
    }
}

fn merge_sort_recursive(
    files: &mut [FileMetadata],
    temp: &mut [FileMetadata],
    left: usize,
    right: usize,
    criteria: SortCriteria,
) -> (usize, usize) {
    if left >= right {
        return (0, 0);
    }
    let mid = left + (right - left) / 2;
    let mut comps = 0;
    let mut swaps = 0;
    
    let (c1, s1) = merge_sort_recursive(files, temp, left, mid, criteria);
    let (c2, s2) = merge_sort_recursive(files, temp, mid + 1, right, criteria);
    let (c3, s3) = merge(files, temp, left, mid, right, criteria);
    
    comps += c1 + c2 + c3;
    swaps += s1 + s2 + s3;
    
    (comps, swaps)
}

fn merge(
    files: &mut [FileMetadata],
    temp: &mut [FileMetadata],
    left: usize,
    mid: usize,
    right: usize,
    criteria: SortCriteria,
) -> (usize, usize) {
    let mut comps = 0;
    let mut swaps = 0;
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;

    while i <= mid && j <= right {
        comps += 1;
        if compare_files(&files[i], &files[j], criteria) != std::cmp::Ordering::Greater {
            temp[k] = files[i].clone();
            i += 1;
        } else {
            temp[k] = files[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        temp[k] = files[i].clone();
        i += 1;
        k += 1;
    }

    while j <= right {
        temp[k] = files[j].clone();
        j += 1;
        k += 1;
    }

    let len = (right + 1) - left;
    files[left..=right].clone_from_slice(&temp[left..=right]);
    swaps += len;

    (comps, swaps)
}
