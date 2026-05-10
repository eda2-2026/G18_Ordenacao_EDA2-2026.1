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
    let comparisons = S::sort(&mut data, criteria);
    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
    BenchmarkResult {
        algorithm: name.to_string(),
        kind: "sort".to_string(),
        duration_ms,
        comparisons,
        swaps: 0,
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
    ]
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
