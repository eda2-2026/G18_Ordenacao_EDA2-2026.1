use crate::core::file_metadata::FileMetadata;

pub trait Searcher {
    fn search(files: &[FileMetadata], query: &str) -> (Vec<usize>, usize);
}
