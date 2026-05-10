use crate::core::file_metadata::FileMetadata;
use crate::sorting::{compare_files, SortCriteria, Sorter};

pub struct HeapSort;

impl Sorter for HeapSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        let len = files.len();
        if len <= 1 {
            return (0, 0);
        }

        let mut comps = 0;
        let mut swaps = 0;

        for i in (0..len / 2).rev() {
            let (c, s) = heapify(files, len, i, criteria);
            comps += c;
            swaps += s;
        }

        for i in (1..len).rev() {
            files.swap(0, i);
            swaps += 1;
            let (c, s) = heapify(files, i, 0, criteria);
            comps += c;
            swaps += s;
        }

        (comps, swaps)
    }
}

fn heapify(files: &mut [FileMetadata], n: usize, i: usize, criteria: SortCriteria) -> (usize, usize) {
    let mut comps = 0;
    let mut swaps = 0;
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n {
        comps += 1;
        if compare_files(&files[left], &files[largest], criteria) == std::cmp::Ordering::Greater {
            largest = left;
        }
    }

    if right < n {
        comps += 1;
        if compare_files(&files[right], &files[largest], criteria) == std::cmp::Ordering::Greater {
            largest = right;
        }
    }

    if largest != i {
        files.swap(i, largest);
        swaps += 1;
        let (c, s) = heapify(files, n, largest, criteria);
        comps += c;
        swaps += s;
    }

    (comps, swaps)
}
