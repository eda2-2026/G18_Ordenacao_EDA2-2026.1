use std::fs::File;
use std::io::Write;
use file_explorer_eda2::core::file_metadata::FileMetadata;
use file_explorer_eda2::sorting::{SortCriteria, Sorter, heap_sort::HeapSort, merge_sort::MergeSort, quick_sort::QuickSort, counting_sort::CountingSort};

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn setup_test_files() -> (std::path::PathBuf, Vec<FileMetadata>) {
    let thread_id = format!("{:?}", std::thread::current().id()).replace(['(', ')'], "");
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = std::env::temp_dir().join(format!(
        "eda2_sort_tests_{}_{}_{}",
        std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
        thread_id,
        count
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    // 1. A dir "Z_Dir"
    std::fs::create_dir(temp_dir.join("Z_Dir")).unwrap();
    // 2. A dir "A_Dir"
    std::fs::create_dir(temp_dir.join("A_Dir")).unwrap();
    // 3. A file "C_File.txt" of size 10
    let mut f1 = File::create(temp_dir.join("C_File.txt")).unwrap();
    f1.write_all(b"0123456789").unwrap();
    let mut f2 = File::create(temp_dir.join("B_File.png")).unwrap();
    f2.write_all(b"12345").unwrap();

    drop(f1);
    drop(f2);

    let items = FileMetadata::list_all_by_path(&temp_dir);
    (temp_dir, items)
}

#[test]
fn test_sorting_algorithms_by_name() {
    let (dir, mut items) = setup_test_files();
    
    let mut items_merge = items.clone();
    let mut items_heap = items.clone();
    
    QuickSort::sort(&mut items, SortCriteria::Nome);
    MergeSort::sort(&mut items_merge, SortCriteria::Nome);
    HeapSort::sort(&mut items_heap, SortCriteria::Nome);
    
    assert_eq!(items[0].name, "A_Dir");
    assert_eq!(items[1].name, "B_File.png");
    assert_eq!(items[2].name, "C_File.txt");
    assert_eq!(items[3].name, "Z_Dir");
    
    assert_eq!(items, items_merge);
    assert_eq!(items, items_heap);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_sorting_algorithms_by_size() {
    let (dir, mut items) = setup_test_files();
    QuickSort::sort(&mut items, SortCriteria::Tamanho);
    
    assert_eq!(items[0].raw_size(), 0);
    assert_eq!(items[1].raw_size(), 0);
    assert_eq!(items[2].raw_size(), 5);
    assert_eq!(items[3].raw_size(), 10);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_sorting_algorithms_by_type() {
    let (dir, mut items) = setup_test_files();
    QuickSort::sort(&mut items, SortCriteria::Tipo);
    
    assert_eq!(items[0].name, "A_Dir");
    assert_eq!(items[1].name, "Z_Dir");
    assert_eq!(items[2].name, "B_File.png");
    assert_eq!(items[3].name, "C_File.txt");

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_counting_sort_by_type() {
    let (dir, mut items) = setup_test_files();
    CountingSort::sort(&mut items, SortCriteria::Tipo);
    
    // Deve ser: A_Dir, Z_Dir, B_File.png, C_File.txt
    assert_eq!(items[0].name, "A_Dir");
    assert_eq!(items[1].name, "Z_Dir");
    assert_eq!(items[2].name, "B_File.png");
    assert_eq!(items[3].name, "C_File.txt");

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn benchmark_quick_vs_heap_10k() {
    let mut items = Vec::with_capacity(10_000);
    // Gerar 10k arquivos simulados com tamanhos "aleatórios"
    for i in 0..10_000 {
        let pseudo_random_size = (i * 137 + 73) % 10_000;
        items.push(FileMetadata::new_mock_for_test(format!("file_{}.txt", i), pseudo_random_size as u64));
    }

    let mut items_quick = items.clone();
    let mut items_heap = items.clone();

    let (comps_q, swaps_q) = QuickSort::sort(&mut items_quick, SortCriteria::Tamanho);
    let (comps_h, swaps_h) = HeapSort::sort(&mut items_heap, SortCriteria::Tamanho);

    println!("Benchmark 10k arquivos por Tamanho:");
    println!("QuickSort: {} comparações, {} trocas", comps_q, swaps_q);
    println!("HeapSort : {} comparações, {} trocas", comps_h, swaps_h);

    assert_eq!(items_quick.len(), 10_000);
    assert_eq!(items_heap.len(), 10_000);

    for i in 0..9_999 {
        assert!(items_quick[i].raw_size() <= items_quick[i+1].raw_size());
        assert!(items_heap[i].raw_size() <= items_heap[i+1].raw_size());
    }
}

#[test]
fn benchmark_counting_vs_merge_10k() {
    let results = file_explorer_eda2::benchmark::run_type_cardinality_benchmark();
    println!("Benchmark 10k arquivos por Tipo (Cardinalidade k ≈ 9):");
    for r in results {
        println!("{}: {} comparações, {} trocas ({} ms)", r.algorithm, r.comparisons, r.swaps, r.duration_ms);
    }
}
