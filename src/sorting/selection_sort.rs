use crate::core::file_metadata::FileMetadata;
use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort(files: &mut Vec<FileMetadata>) {
        let n = files.len();
        for i in 0..n {
            let mut min = i;
            for j in i + 1..n {
                if files[j].name < files[min].name {
                    min = j;
                }
            }
            if min != i {
                files.swap(i, min);
            }
        }
    }
}
