use std::fs::File;
use std::io::Write;
use file_explorer_eda2::core::file_metadata::FileMetadata;
use file_explorer_eda2::search::{Searcher, BinarySearch, linear_search::LinearSearch, recursive_search::RecursiveSearch};
use file_explorer_eda2::sorting::{SortCriteria, Sorter, quick_sort::QuickSort};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn setup_test_files() -> (std::path::PathBuf, Vec<FileMetadata>) {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = std::env::temp_dir().join(format!("eda2_srch_dir_{}_{}", std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis(), count));
    std::fs::create_dir_all(&temp_dir).unwrap();

    std::fs::create_dir(temp_dir.join("Folder_A")).unwrap();
    std::fs::create_dir(temp_dir.join("Folder_B")).unwrap();
    
    let mut f1 = File::create(temp_dir.join("Test_File_1.txt")).unwrap();
    f1.write_all(b"Hello").unwrap();
    
    let mut f2 = File::create(temp_dir.join("Folder_A").join("Test_File_2.png")).unwrap();
    f2.write_all(b"Image").unwrap();

    drop(f1);
    drop(f2);

    let items = FileMetadata::list_all_by_path(&temp_dir);
    (temp_dir, items)
}

fn setup_search_files() -> (std::path::PathBuf, Vec<FileMetadata>) {
    let thread_id = format!("{:?}", std::thread::current().id()).replace(['(', ')'], "");
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = std::env::temp_dir().join(format!(
        "eda2_binsearch_tests_{}_{}_{}",
        std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
        thread_id,
        count
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    std::fs::create_dir(temp_dir.join("Banana_Dir")).unwrap();
    std::fs::create_dir(temp_dir.join("Alpha_Dir")).unwrap();
    File::create(temp_dir.join("Banana_Notes.txt")).unwrap().write_all(b"content").unwrap();
    File::create(temp_dir.join("Carrot.png")).unwrap().write_all(b"img").unwrap();
    File::create(temp_dir.join("Apple.rs")).unwrap().write_all(b"fn main() {}").unwrap();

    let mut items = FileMetadata::list_all_by_path(&temp_dir);
    QuickSort::sort(&mut items, SortCriteria::Nome);
    (temp_dir, items)
}

#[test]
fn test_linear_search() {
    let (dir, items) = setup_test_files();
    
    let (matches_txt, comps1) = LinearSearch::search(&items, "txt");
    assert_eq!(matches_txt.len(), 1);
    assert_eq!(items[matches_txt[0]].name, "Test_File_1.txt");
    assert_eq!(comps1, items.len());

    let (matches_folder, _) = LinearSearch::search(&items, "Folder");
    assert_eq!(matches_folder.len(), 2);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_recursive_search() {
    let (dir, _items) = setup_test_files();
    
    let (all_files, matches_png, comps) = RecursiveSearch::search(&dir, "png");
    
    assert_eq!(matches_png.len(), 1);
    assert_eq!(all_files[matches_png[0]].name, "Test_File_2.png");
    assert_eq!(comps, all_files.len());

    let (_, matches_test, _) = RecursiveSearch::search(&dir, "Test");
    assert_eq!(matches_test.len(), 2);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_prefix_multiplos_resultados() {
    let (dir, items) = setup_search_files();
    let (indices, comparisons) = BinarySearch::search(&items, "Ban");
    assert_eq!(indices.len(), 2);
    assert!(indices.iter().all(|&i| items[i].name.to_lowercase().starts_with("ban")));
    assert!(comparisons > 0);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_sem_resultado() {
    let (dir, items) = setup_search_files();
    let (indices, _) = BinarySearch::search(&items, "Zucchini");
    assert!(indices.is_empty());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_nome_exato() {
    let (dir, items) = setup_search_files();
    let (indices, _) = BinarySearch::search(&items, "Carrot.png");
    assert_eq!(indices.len(), 1);
    assert_eq!(items[indices[0]].name, "Carrot.png");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_query_vazia() {
    let (dir, items) = setup_search_files();
    let (indices, comparisons) = BinarySearch::search(&items, "");
    assert!(indices.is_empty());
    assert_eq!(comparisons, 0);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_case_insensitive() {
    let (dir, items) = setup_search_files();
    let (lower, _) = BinarySearch::search(&items, "alpha");
    let (upper, _) = BinarySearch::search(&items, "ALPHA");
    assert_eq!(lower, upper);
    assert_eq!(lower.len(), 1);
    assert_eq!(items[lower[0]].name, "Alpha_Dir");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_binary_search_lista_vazia() {
    let (indices, comparisons) = BinarySearch::search(&[], "query");
    assert!(indices.is_empty());
    assert_eq!(comparisons, 0);
}
