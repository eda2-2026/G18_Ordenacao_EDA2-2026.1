use crate::core::file_metadata::FileMetadata;

pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub use bubble_sort::BubbleSort;
pub use selection_sort::SelectionSort;
pub use insertion_sort::InsertionSort;

pub enum SortCriteria {
    Nome,
    Tamanho,
    Data,
    Tipo,
}

pub trait Sorter {
    fn sort(files: &mut Vec<FileMetadata>);
}
