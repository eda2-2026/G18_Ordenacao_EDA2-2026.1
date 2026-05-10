use crate::core::file_metadata::FileMetadata;

pub enum SortCriteria {
    Nome,
    Tamanho,
    Data,
    Tipo,
}

pub trait Sorter {
    fn sort(files: &mut Vec<FileMetadata>);
}
