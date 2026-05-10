# G18 — File Explorer com Ordenação e Busca (EDA2 2026.1)

Projeto acadêmico do Grupo 18 para a disciplina de Estruturas de Dados e Algoritmos 2.

## Objetivo

Aplicar e comparar algoritmos de **ordenação** e **busca** em um explorador de arquivos real,
medindo desempenho e eficiência em cenários práticos.

## Análise e Comparativo de Algoritmos

Após a implementação dos algoritmos, consolidamos um relatório prático focando em **"qual algoritmo escolher de acordo com a situação"**. Os resultados comparativos detalhados, os benchmarks contra os cenários mapeados e a conclusão acadêmica estão na seção:
👉 [**Análise de Algoritmos**](ANALISE_ALGORITMOS.md)

## Apresentação em Vídeo do Projeto:
- https://youtu.be/0Aonp0l2pCY

## Stack

- **Linguagem:** Rust.
- **UI:** [Slint](https://slint.dev/) (frontend reutilizado do projeto [windows-fast-file-explorer](https://github.com/Bappoz/windows-fast-file-explorer))
- **Sistema de arquivos:** WalkDir + rayon (paralelismo)

## Estrutura

```
src/
├── main.rs                    # Entrada da aplicação (UI + callbacks)
├── lib.rs                     # Re-exports públicos da lib
│
├── core/
│   └── file_metadata.rs       # Modelo de dados de arquivo/diretório
│
├── sorting/                   # Módulo de algoritmos de ordenação
│   ├── mod.rs                 # Re-exports + documentação acadêmica
│   ├── common.rs              # Trait Sorter + SortCriteria + compare_files
│   ├── bubble_sort.rs         # Bubble Sort   — O(n²)
│   ├── selection_sort.rs      # Selection Sort — O(n²), O(n) trocas
│   ├── insertion_sort.rs      # Insertion Sort — O(n²), O(n) quasi-sorted
│   ├── quick_sort.rs          # Quick Sort    — O(n log n) médio, in-place
│   ├── merge_sort.rs          # Merge Sort    — O(n log n) estável
│   ├── heap_sort.rs           # Heap Sort     — O(n log n) garantido, O(1) memória
│   └── counting_sort.rs       # Counting Sort — O(n+k), não-comparativo (Tipo)
│
├── search/                    # Módulo de algoritmos de busca
│   ├── mod.rs                 # Re-exports + trait Searcher
│   ├── common.rs              # Tipos compartilhados de busca
│   ├── linear_search.rs       # Busca Linear  — O(n)
│   ├── binary_search.rs       # Busca Binária — O(log n)
│   └── recursive_search.rs    # Busca Recursiva em subpastas — WalkDir
│
├── benchmark/
│   └── mod.rs                 # Suíte de benchmarks (sort + busca + estabilidade)
│
├── bin/
│   └── synthesis.rs           # Binário de síntese acadêmica → gera ANALISE_ALGORITMOS.md
│
└── view/                      # UI Slint (frontend)
    ├── app-window.slint        # Janela principal + painel de métricas
    ├── componets/
    │   ├── file-item.slint     # Item individual da lista de arquivos
    │   ├── file-list.slint     # Lista de arquivos
    │   ├── sidebar.slint       # Painel lateral (favoritos/locais)
    │   ├── statusbar.slint     # Barra de status (métricas de sort/busca)
    │   └── toolbar.slint       # Toolbar (critério de ordenação + campo de busca)
    └── styles/
        └── theme.slint         # Design system / tokens de cor e tipografia

tests/
├── sorting_tests.rs            # Testes de ordenação + benchmarks de validação
└── search_tests.rs             # Testes de busca linear, binária e recursiva

ANALISE_ALGORITMOS.md           # Relatório gerado automaticamente pelo bin/synthesis
```

## Setup

```bash
cargo run
```

> Requer Rust 1.85+ (edition 2024)

## Grupo

| Membro | GitHub |
|--------|--------|
| Heitor Macedo | [HeitorM50](https://github.com/HeitorM50) |
| Lucas Zanetti | [Bappoz](https://github.com/Bappoz) |

