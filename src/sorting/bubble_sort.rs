use crate::core::file_metadata::FileMetadata;
use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort(files: &mut Vec<FileMetadata>) {
        let n = files.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                if files[j].name > files[j + 1].name {
                    files.swap(j, j + 1);
                }
            }
        }
    }
}
