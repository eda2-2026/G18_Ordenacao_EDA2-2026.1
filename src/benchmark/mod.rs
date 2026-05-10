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
    let files: Vec<FileMetadata> = (0..n)
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

/// Cria N arquivos mock com timestamps interleaved: ~`tie_fraction` deles compartilham o mesmo
/// timestamp (`TIE_TS`), o restante tem timestamps únicos crescentes. O padrão intercalado
/// desafia o Quick Sort a expor instabilidade nos grupos de empate.
fn make_with_repeated_timestamps(n: usize, tie_fraction: f64) -> Vec<FileMetadata> {
    const TIE_TS: u64 = 1_000_000;
    // period=3 para ~33% de empates (suficiente para demonstrar 30%)
    let period = (1.0_f64 / tie_fraction).round().max(2.0) as usize;
    let mut uniq_ts = TIE_TS + 1;
    let mut tie_idx = 0usize;

    (0..n)
        .map(|i| {
            if i % period == 0 {
                let f = FileMetadata::new_mock_with_date(
                    format!("tie_{:06}", tie_idx),
                    1024,
                    TIE_TS,
                );
                tie_idx += 1;
                f
            } else {
                let f = FileMetadata::new_mock_with_date(
                    format!("uniq_{:06}", uniq_ts - TIE_TS - 1),
                    1024,
                    uniq_ts,
                );
                uniq_ts += 1;
                f
            }
        })
        .collect()
}

/// Verifica se `sorted` é uma ordenação estável de `original` por `raw_modified()`.
///
/// Para cada grupo de arquivos com o mesmo timestamp em `sorted`, checa se a ordem
/// relativa entre eles corresponde à sua posição em `original`. Retorna `false` se
/// alguma inversão for detectada.
pub fn verify_sort_stability(original: &[FileMetadata], sorted: &[FileMetadata]) -> bool {
    let orig_pos: std::collections::HashMap<&str, usize> = original
        .iter()
        .enumerate()
        .map(|(i, f)| (f.name.as_str(), i))
        .collect();

    let n = sorted.len();
    let mut i = 0;
    while i < n {
        let ts = sorted[i].raw_modified();
        let mut j = i + 1;
        while j < n && sorted[j].raw_modified() == ts {
            j += 1;
        }
        // grupo sorted[i..j] compartilha o mesmo timestamp — verifica ordem relativa
        if j > i + 1 {
            let positions: Vec<usize> = sorted[i..j]
                .iter()
                .map(|f| orig_pos[f.name.as_str()])
                .collect();
            if positions.windows(2).any(|w| w[0] >= w[1]) {
                return false;
            }
        }
        i = j;
    }
    true
}

/// Benchmark de estabilidade para `SortCriteria::Data` com 30% de arquivos com timestamp idêntico.
///
/// Para cada N em [100, 1_000, 10_000], cria um dataset interleaved onde ~30% dos arquivos
/// compartilham o mesmo timestamp e executa Quick Sort, Merge Sort e Insertion Sort.
/// Imprime no stdout se cada algoritmo preservou a ordem relativa dos empates.
pub fn run_stability_date_benchmark() -> Vec<BenchmarkResult> {
    let sizes = [100usize, 1_000, 10_000];
    const TIE_FRACTION: f64 = 0.30;
    let mut results = Vec::new();

    for &n in &sizes {
        let original = make_with_repeated_timestamps(n, TIE_FRACTION);
        let kind = format!("stability_date_30pct_ties_n{n}");

        let mut qs = original.clone();
        let t = Instant::now();
        let (qs_comps, qs_swaps) = QuickSort::sort(&mut qs, SortCriteria::Data);
        let qs_ms = t.elapsed().as_secs_f64() * 1000.0;
        let qs_stable = verify_sort_stability(&original, &qs);

        let mut ms = original.clone();
        let t = Instant::now();
        let (ms_comps, ms_swaps) = MergeSort::sort(&mut ms, SortCriteria::Data);
        let ms_ms = t.elapsed().as_secs_f64() * 1000.0;
        let ms_stable = verify_sort_stability(&original, &ms);

        let mut is = original.clone();
        let t = Instant::now();
        let (is_comps, is_swaps) = InsertionSort::sort(&mut is, SortCriteria::Data);
        let is_ms = t.elapsed().as_secs_f64() * 1000.0;
        let is_stable = verify_sort_stability(&original, &is);

        println!("=== Stability Date Benchmark (n={n}, ~30% ties) ===");
        println!("  QuickSort:     {qs_ms:.4}ms, {qs_comps} comps, {qs_swaps} swaps — estavel={qs_stable}");
        println!("  MergeSort:     {ms_ms:.4}ms, {ms_comps} comps, {ms_swaps} swaps — estavel={ms_stable}");
        println!("  InsertionSort: {is_ms:.4}ms, {is_comps} comps, {is_swaps} swaps — estavel={is_stable}");

        results.push(BenchmarkResult { algorithm: "QuickSort".into(),     kind: kind.clone(), duration_ms: qs_ms, comparisons: qs_comps, swaps: qs_swaps });
        results.push(BenchmarkResult { algorithm: "MergeSort".into(),     kind: kind.clone(), duration_ms: ms_ms, comparisons: ms_comps, swaps: ms_swaps });
        results.push(BenchmarkResult { algorithm: "InsertionSort".into(), kind: kind.clone(), duration_ms: is_ms, comparisons: is_comps, swaps: is_swaps });
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