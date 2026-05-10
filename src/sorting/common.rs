use crate::core::file_metadata::FileMetadata;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum SortCriteria {
    Nome,
    Tamanho,
    Data,
    Tipo,
}

pub trait Sorter {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> usize;
}

pub fn compare_files(a: &FileMetadata, b: &FileMetadata, criteria: SortCriteria) -> Ordering {
    match criteria {
        SortCriteria::Nome => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        SortCriteria::Tamanho => a.raw_size().cmp(&b.raw_size()),
        SortCriteria::Data => a.raw_modified().cmp(&b.raw_modified()),
        SortCriteria::Tipo => {
            if a.is_dir() && !b.is_dir() {
                Ordering::Less
            } else if !a.is_dir() && b.is_dir() {
                Ordering::Greater
            } else if a.is_dir() && b.is_dir() {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                let cmp_ext = a.extension().to_lowercase().cmp(&b.extension().to_lowercase());
                if cmp_ext == Ordering::Equal {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                } else {
                    cmp_ext
                }
            }
        }
    }
}
