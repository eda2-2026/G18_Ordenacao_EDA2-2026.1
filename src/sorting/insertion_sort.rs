use crate::core::file_metadata::FileMetadata;
use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort(files: &mut Vec<FileMetadata>) {
        let n = files.len();
        for i in 1..n {
            let mut j = i;
            while j > 0 && files[j - 1].name > files[j].name {
                files.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}
