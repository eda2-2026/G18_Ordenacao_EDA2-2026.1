/*!
# Módulo de Ordenação

Este módulo implementa algoritmos de ordenação para o File Explorer.

## Análise de Algoritmos para Dados Numéricos

Para a ordenação por tamanho numérico (`SortCriteria::Tamanho`, baseado em `u64`), o **Quick Sort** demonstrou ser o algoritmo mais adequado.

Quando lidamos com campos numéricos de acesso direto (como `u64`), a comparação é $O(1)$ e extremamente barata. O gargalo real no tempo de execução desloca-se para:
1. Número de trocas (swaps) em memória.
2. Localidade de referência (cache-friendliness).

### Por que o Quick Sort?
- **Quick Sort (in-place):** Faz varreduras lineares da esquerda para a direita e da direita para a esquerda. Esse comportamento sequencial minimiza *cache misses* (falhas de cache do processador). Ele exige $O(\log n)$ de memória na pilha, e executa um número otimizado de trocas.
- **Heap Sort:** Garante $O(n \log n)$ em tempo com $O(1)$ espaço, independentemente da entrada. Contudo, seu padrão de salto de índices (navegando da raiz para as folhas saltando posições) resulta em inúmeros *cache misses*, gerando um custo constante prático mais alto que o Quick Sort.
- **Merge Sort:** Excelente para dados instáveis, mas sua necessidade de $O(n)$ memória extra (ou o equivalente a mover itens entre arrays temporários repetidamente) introduz alto *overhead* que prejudica seu desempenho em números estritamente numéricos que poderiam ser trocados `in-place`.
- **Selection/Bubble Sort:** Fazem comparações quadráticas $O(n^2)$, destruindo a escalabilidade na casa de 10k arquivos ou mais. O Selection até faz $O(n)$ trocas na memória, mas sucumbe no número astronômico de comparações.

Portanto, por sua eficiência teórica e localidade no acesso físico aos dados, o **Quick Sort** é a implementação preferida para esse caso numérico no file explorer.
*/

pub mod common;
pub mod heap_sort;
pub mod merge_sort;
pub mod quick_sort;

pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub use common::*;

pub use bubble_sort::BubbleSort;
pub use selection_sort::SelectionSort;
pub use insertion_sort::InsertionSort;
