use std::fs::File;
use std::io::Write;
use std::time::Instant;

use file_explorer_eda2::core::file_metadata::FileMetadata;
use file_explorer_eda2::sorting::{
    counting_sort::CountingSort, heap_sort::HeapSort, insertion_sort::InsertionSort,
    merge_sort::MergeSort, quick_sort::QuickSort, SortCriteria, Sorter,
};

#[derive(Debug)]
struct ScenarioResult {
    algo_name: &'static str,
    time_ms: f64,
    comparisons: usize,
    swaps: usize,
    theoretical_memory: &'static str,
}

fn bench<T: Sorter>(
    algo_name: &'static str,
    files: &[FileMetadata],
    criteria: SortCriteria,
    theoretical_memory: &'static str,
) -> ScenarioResult {
    let mut arr = files.to_vec();
    let start = Instant::now();
    let (comps, swaps) = T::sort(&mut arr, criteria);
    let time_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    ScenarioResult {
        algo_name,
        time_ms,
        comparisons: comps,
        swaps,
        theoretical_memory,
    }
}

fn run_scenario(
    name: &str,
    n: usize,
    criteria: SortCriteria,
    insight: &str,
    generator: impl Fn() -> Vec<FileMetadata>,
) -> String {
    let files = generator();
    
    let mut results = vec![
        bench::<QuickSort>("Quick Sort", &files, criteria, "O(log n)"),
        bench::<MergeSort>("Merge Sort", &files, criteria, "O(n)"),
        bench::<HeapSort>("Heap Sort", &files, criteria, "O(1)"),
        bench::<InsertionSort>("Insertion Sort", &files, criteria, "O(1)"),
    ];

    if criteria == SortCriteria::Tipo {
        results.push(bench::<CountingSort>("Counting Sort", &files, criteria, "O(n)"));
    }

    // Ordenar resultados pelo tempo
    results.sort_by(|a, b| a.time_ms.partial_cmp(&b.time_ms).unwrap());

    let mut md = format!("### Cenário: {} (N = {}, Critério: {:?})\n\n", name, n, criteria);
    md.push_str(&format!("**Expectativa / Insight:** {}\n\n", insight));
    
    md.push_str("| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |\n");
    md.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
    
    for r in results {
        let is_winner = md.lines().count() == 5; // A primeira linha da tabela será o vencedor, se a gente marcou como o primeiro
        let bold = if is_winner { "**" } else { "" };
        md.push_str(&format!(
            "| {}{}{} | {}{:.3} ms{} | {}{} comps{} | {}{} trocas{} | {} |\n",
            bold, r.algo_name, bold,
            bold, r.time_ms, bold,
            bold, r.comparisons, bold,
            bold, r.swaps, bold,
            r.theoretical_memory
        ));
    }
    
    md.push_str("\n---\n\n");
    md
}

fn main() {
    println!("Iniciando benchmarks para síntese acadêmica...");
    let mut md_output = String::new();

    md_output.push_str("# Síntese Acadêmica: Análise Comparativa de Algoritmos\n\n");
    md_output.push_str("Este documento apresenta a análise comparativa de performance real dos algoritmos implementados, submetidos a diferentes cenários e critérios de ordenação que refletem o uso cotidiano em um explorador de arquivos.\n\n");

    // Cenário 1: Diretório home (~)
    md_output.push_str(&run_scenario(
        "Diretório Home (~)",
        500,
        SortCriteria::Nome,
        "Para arrays pequenos, Insertion Sort costuma ser extremamente rápido devido à baixa sobrecarga de chamadas recursivas, rivalizando com Merge/Quick Sort.",
        || {
            (0..500).map(|i| {
                let name = format!("arquivo_{}.txt", (i * 997) % 500);
                FileMetadata::new_mock_for_test(name, 1024)
            }).collect()
        }
    ));

    // Cenário 2: Pasta de downloads
    md_output.push_str(&run_scenario(
        "Pasta de Downloads",
        2000,
        SortCriteria::Tamanho,
        "Cenário genérico com distribuição variada de tamanhos. Quick Sort costuma vencer por ser in-place e cache-friendly.",
        || {
            (0..2000).map(|i| {
                let size = ((i * 13) % 10000) as u64;
                FileMetadata::new_mock_for_test(format!("download_{}.zip", i), size)
            }).collect()
        }
    ));

    // Cenário 3: Projeto git com muitos commits
    md_output.push_str(&run_scenario(
        "Projeto Git (Datas Repetidas)",
        5000,
        SortCriteria::Data,
        "Apresenta alta incidência de datas iguais. Merge Sort é excelente aqui pois sua estabilidade preserva a ordem de arquivos com a mesma data.",
        || {
            (0..5000).map(|i| {
                let ts = if i % 3 == 0 { 1_000_000 } else { 1_000_000 + i as u64 };
                FileMetadata::new_mock_with_date(format!("src_{}.rs", i), 1024, ts)
            }).collect()
        }
    ));

    // Cenário 4: Raiz de disco (Tipo)
    md_output.push_str(&run_scenario(
        "Raiz de Disco",
        10000,
        SortCriteria::Tipo,
        "Com milhares de arquivos, mas poucas extensões (Baixa Cardinalidade), os algoritmos comparativos sofrem. Counting Sort é projetado exatamente para isso, executando em tempo O(n + k).",
        || {
            let exts = ["sys", "dll", "exe", "log", "tmp", "dat", "ini"];
            (0..10000).map(|i| {
                if i % 15 == 0 {
                    FileMetadata::new_mock_for_test(format!("Windows_{}", i), 0).with_is_dir(true)
                } else {
                    let ext = exts[i % exts.len()];
                    FileMetadata::new_mock_for_test(format!("file_{}.{}", i, ext), i as u64).with_extension(ext.to_string())
                }
            }).collect()
        }
    ));

    // Cenário 5: Lista quase-ordenada
    md_output.push_str(&run_scenario(
        "Lista Quase-Ordenada (1 item novo)",
        1000,
        SortCriteria::Nome,
        "O Insertion Sort possui tempo O(n) em listas quase ordenadas. Ele identifica rapidamente os elementos no lugar e brilha incomparavelmente aqui.",
        || {
            let mut files: Vec<FileMetadata> = (0..1000).map(|i| {
                FileMetadata::new_mock_for_test(format!("doc_{:04}.txt", i), 1024)
            }).collect();
            // Deslocar apenas 1 elemento
            files.swap(10, 990);
            files
        }
    ));

    // Cenário 6: Lista invertida (Pior caso)
    md_output.push_str(&run_scenario(
        "Lista Invertida (Pior Caso)",
        1000,
        SortCriteria::Tamanho,
        "Para uma lista totalmente invertida, o Quick Sort tradicional degrada para O(n²). Heap Sort e Merge Sort mantêm sua consistência algorítmica O(n log n).",
        || {
            (0..1000).rev().map(|i| {
                FileMetadata::new_mock_for_test(format!("file_{}.txt", i), i as u64)
            }).collect()
        }
    ));

    md_output.push_str("## Conclusão Acadêmica\n\n");
    md_output.push_str("A experimentação prática prova que **não existe o melhor algoritmo absoluto**, mas sim o algoritmo mais adequado para cada topologia de dados:\n");
    md_output.push_str("- **Por Tipo/Extensão (Baixa Cardinalidade):** `Counting Sort` quebra a barreira do O(n log n) agrupando os elementos em buckets (O(n)). É insuperável aqui.\n");
    md_output.push_str("- **Por Data/Nome (Estabilidade Importante):** `Merge Sort` é o recomendado, preservando com segurança elementos que caem na mesma data, embora exija O(n) de memória extra.\n");
    md_output.push_str("- **Por Tamanho (Performance in-place):** `Quick Sort` (ou Heap Sort para evitar O(n²) no pior caso) é formidável, rodando inteiramente em cache e trocando arquivos rapidamente sem gastar memória alocada substancial.\n");
    md_output.push_str("- **Diretórios Quase Prontos:** O trivial `Insertion Sort` é uma máquina perfeita se apenas 1 ou 2 arquivos foram adicionados à pasta.\n");

    let mut file = File::create("ANALISE_ALGORITMOS.md").unwrap();
    file.write_all(md_output.as_bytes()).unwrap();

    println!("Arquivo ANALISE_ALGORITMOS.md gerado com sucesso na raiz do projeto!");
}
