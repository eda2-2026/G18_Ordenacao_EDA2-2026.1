/*!
# Módulo de Ordenação

Este módulo implementa algoritmos de ordenação para o File Explorer.

## Análise de Algoritmos para Dados Numéricos (`SortCriteria::Tamanho`)

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

---

## Análise de Algoritmos para Strings (`SortCriteria::Nome`)

A ordenação por nome tem características distintas que impactam a escolha do algoritmo:

- Comparações são **lexicográficas** (strings), $O(k)$ onde $k$ é o comprimento médio do nome — muito mais custosas que inteiros.
- Usuários frequentemente re-ordenam uma lista **já quase-ordenada** (ex: adicionou 1 ou 2 arquivos novos ao diretório).
- Há expectativa de **estabilidade**: dois arquivos com nomes iguais devem manter ordem relativa anterior.

### Tabela comparativa

| Algoritmo      | Quase-ordenado | Pior caso    | Estável? | Observação                             |
|----------------|---------------|--------------|----------|----------------------------------------|
| Insertion Sort | **O(n)**      | O(n²)        | ✅        | Ideal para re-ordenação incremental    |
| Merge Sort     | O(n log n)    | O(n log n)   | ✅        | Preferível para listas grandes aleatórias |
| Quick Sort     | O(n log n)    | O(n²)        | ❌        | Evitar como padrão para nomes          |
| Bubble Sort    | O(n)          | O(n²)        | ✅        | Apenas didático/benchmark              |

### Recomendação para `SortCriteria::Nome`

**Para listas quase-ordenadas (caso dominante em exploradores de arquivos):**
O **Insertion Sort** é a escolha ótima. Quando apenas alguns arquivos estão fora de posição,
o número de inversões é mínimo e o algoritmo executa em tempo próximo a $O(n)$, superando
o Merge Sort apesar de ser $O(n²)$ no pior caso. O custo de comparação de strings torna o
overhead do Merge Sort (movimentação de $O(n)$ elementos extra em cada nível de recursão)
mais significativo.

**Para listas grandes sem garantia de ordenação prévia:**
O **Merge Sort** é mais robusto — garante $O(n \log n)$ independentemente da distribuição
de entrada e mantém estabilidade.

**Hipótese verificada por benchmark** (`run_nearly_sorted_name_benchmark`):
Para N = 100/1000/10000 arquivos com apenas 1 elemento fora de posição,
o Insertion Sort supera o Merge Sort em tempo de execução e número de comparações.
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
