use crate::core::file_metadata::FileMetadata;
use crate::sorting::{compare_files, SortCriteria, Sorter};

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> usize {
        let mut comparisons = 0;
        let len = files.len();
        if len > 1 {
            let mut temp = files.clone();
            comparisons += merge_sort_recursive(files, &mut temp, 0, len - 1, criteria);
        }
        comparisons
    }
}

fn merge_sort_recursive(
    files: &mut [FileMetadata],
    temp: &mut [FileMetadata],
    left: usize,
    right: usize,
    criteria: SortCriteria,
) -> usize {
    if left >= right {
        return 0;
    }
    let mid = left + (right - left) / 2;
    let mut comps = 0;
    comps += merge_sort_recursive(files, temp, left, mid, criteria);
    comps += merge_sort_recursive(files, temp, mid + 1, right, criteria);
    comps += merge(files, temp, left, mid, right, criteria);
    comps
}

fn merge(
    files: &mut [FileMetadata],
    temp: &mut [FileMetadata],
    left: usize,
    mid: usize,
    right: usize,
    criteria: SortCriteria,
) -> usize {
    let mut comps = 0;
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

    for idx in left..=right {
        files[idx] = temp[idx].clone();
    }

    comps
}
