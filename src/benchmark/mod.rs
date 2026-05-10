use std::fs::File;
use std::io::Write;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::core::file_metadata::FileMetadata;
use crate::search::binary_search::BinarySearch;
use crate::search::common::Searcher;
use crate::search::linear_search::LinearSearch;
use crate::sorting::bubble_sort::BubbleSort;
use crate::sorting::heap_sort::HeapSort;
use crate::sorting::insertion_sort::InsertionSort;
use crate::sorting::merge_sort::MergeSort;
use crate::sorting::quick_sort::QuickSort;
use crate::sorting::selection_sort::SelectionSort;
use crate::sorting::counting_sort::CountingSort;
use crate::sorting::{SortCriteria, Sorter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub algorithm: String,
    pub kind: String,
    pub duration_ms: f64,
    pub comparisons: usize,
    pub swaps: usize,
}

impl BenchmarkResult {
    fn csv_row(&self) -> String {
        format!(
            "{},{},{:.4},{},{}",
            self.algorithm, self.kind, self.duration_ms, self.comparisons, self.swaps
        )
    }
}

const CSV_HEADER: &str = "algorithm,kind,duration_ms,comparisons,swaps";

fn bench_sorter<S: Sorter>(
    name: &str,
    files: &[FileMetadata],
    criteria: SortCriteria,
) -> BenchmarkResult {
    let mut data = files.to_vec();
    let start = Instant::now();
    let (comparisons, swaps) = S::sort(&mut data, criteria);
    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
    BenchmarkResult {
        algorithm: name.to_string(),
        kind: "sort".to_string(),
        duration_ms,
        comparisons,
        swaps,
    }
}

fn bench_searcher<S: Searcher>(
    name: &str,
    files: &[FileMetadata],
    query: &str,
) -> BenchmarkResult {
    let start = Instant::now();
    let (_, comparisons) = S::search(files, query);
    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
    BenchmarkResult {
        algorithm: name.to_string(),
        kind: "search".to_string(),
        duration_ms,
        comparisons,
        swaps: 0,
    }
}

pub fn run_sort_benchmarks(files: &[FileMetadata], criteria: SortCriteria) -> Vec<BenchmarkResult> {
    vec![
        bench_sorter::<BubbleSort>("BubbleSort", files, criteria),
        bench_sorter::<SelectionSort>("SelectionSort", files, criteria),
        bench_sorter::<InsertionSort>("InsertionSort", files, criteria),
        bench_sorter::<QuickSort>("QuickSort", files, criteria),
        bench_sorter::<MergeSort>("MergeSort", files, criteria),
        bench_sorter::<HeapSort>("HeapSort", files, criteria),
        bench_sorter::<CountingSort>("CountingSort", files, criteria),
    ]
}

/// Gera 10.000 arquivos variando entre diretórios e algumas extensões comuns.
pub fn run_type_cardinality_benchmark() -> Vec<BenchmarkResult> {
    let extensions = ["txt", "png", "rs", "toml", "md", "jpg", "pdf", "docx"];
    let n = 10_000;
    let mut files: Vec<FileMetadata> = (0..n)
        .map(|i| {
            if i % 10 == 0 {
                // Diretório
                FileMetadata::new_mock_for_test(format!("dir_{:05}", i), 0).with_is_dir(true)
            } else {
                // Arquivo com extensão
                let ext = extensions[i % extensions.len()];
                FileMetadata::new_mock_for_test(format!("file_{:05}.{}", i, ext), i as u64).with_extension(ext.to_string())
            }
        })
        .collect();

    let mut results = Vec::new();

    let mut merge = bench_sorter::<MergeSort>("MergeSort", &files, SortCriteria::Tipo);
    merge.kind = "type_cardinality_10k".to_string();
    results.push(merge);

    let mut counting = bench_sorter::<CountingSort>("CountingSort", &files, SortCriteria::Tipo);
    counting.kind = "type_cardinality_10k".to_string();
    results.push(counting);

    results
}

/// Gera uma lista de N arquivos quase-ordenados por nome, com 1 elemento fora de posição.
/// O último elemento (lexicograficamente o maior) é movido para o início,
/// simulando a adição de um novo arquivo em um diretório já ordenado.
fn make_nearly_sorted_by_name(n: usize) -> Vec<FileMetadata> {
    let mut files: Vec<FileMetadata> = (0..n)
        .map(|i| FileMetadata::new_mock_for_test(format!("file_{:06}", i), i as u64))
        .collect();
    // move o último para o início — cria exatamente n-1 inversões
    let last = files.remove(n - 1);
    files.insert(0, last);
    files
}

/// Benchmark específico para `SortCriteria::Nome` com listas quase-ordenadas.
///
/// Compara InsertionSort vs MergeSort para N = 100, 1_000 e 10_000 arquivos,
/// cada lista com apenas 1 elemento fora de posição (cenário de re-ordenação incremental).
/// Verifica a hipótese: InsertionSort supera MergeSort quando o número de inversões é mínimo.
pub fn run_nearly_sorted_name_benchmark() -> Vec<BenchmarkResult> {
    let sizes = [100usize, 1_000, 10_000];
    let mut results = Vec::with_capacity(sizes.len() * 2);

    for &n in &sizes {
        let files = make_nearly_sorted_by_name(n);
        let label_suffix = format!("_n{n}_nearly_sorted_name");

        let mut insertion = bench_sorter::<InsertionSort>(
            &format!("InsertionSort{label_suffix}"),
            &files,
            SortCriteria::Nome,
        );
        insertion.kind = "nearly_sorted_name".to_string();

        let mut merge = bench_sorter::<MergeSort>(
            &format!("MergeSort{label_suffix}"),
            &files,
            SortCriteria::Nome,
        );
        merge.kind = "nearly_sorted_name".to_string();

        results.push(insertion);
        results.push(merge);
    }

    results
}

pub fn run_search_benchmarks(files: &[FileMetadata], query: &str) -> Vec<BenchmarkResult> {
    let mut sorted = files.to_vec();
    sorted.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    vec![
        bench_searcher::<LinearSearch>("LinearSearch", files, query),
        bench_searcher::<BinarySearch>("BinarySearch", &sorted, query),
    ]
}

pub fn save_to_csv(results: &[BenchmarkResult], path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    writeln!(f, "{CSV_HEADER}")?;
    for r in results {
        writeln!(f, "{}", r.csv_row())?;
    }
    Ok(())
}

pub fn save_to_json(results: &[BenchmarkResult], path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(results)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut f = File::create(path)?;
    f.write_all(json.as_bytes())
}