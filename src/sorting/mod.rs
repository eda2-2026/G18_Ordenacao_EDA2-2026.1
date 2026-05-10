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

---

## Análise de Cardinalidade e Algoritmos Não-Comparativos (`SortCriteria::Tipo`)

A ordenação por tipo (diretório vs arquivo) e extensão demonstra a importância do conceito de **cardinalidade**. A cardinalidade é o número de valores distintos que uma chave de ordenação pode assumir.

- **Baixa Cardinalidade**: `SortCriteria::Tipo` separa apenas entre 2 categorias (Diretório e Arquivo), ou algumas dezenas se considerarmos extensões únicas.
- **Alta Cardinalidade**: `SortCriteria::Tamanho` ou `SortCriteria::Nome` (praticamente todos os valores são únicos).

Quando a cardinalidade $k$ é muito menor que o número de elementos $N$ ($k \ll N$), algoritmos baseados em comparações ($O(n \log n)$) fazem um trabalho ineficiente, pois perdem tempo comparando elementos que poderiam ser simplesmente agrupados. 

### O Poder do Counting Sort
Neste cenário, podemos quebrar o limite inferior de $O(n \log n)$ utilizando um **algoritmo não-comparativo** como o **Counting Sort**. 

A estratégia para `SortCriteria::Tipo` é:
1. Mapear as categorias (Diretório = 0, Extensões = 1..k).
2. Distribuir os elementos em seus baldes (buckets/counts) correspondentes.
3. Isso exige uma varredura para contar e outra para distribuir, resultando em um tempo real de **$O(n + k)$**.

Para 10.000 itens (com apenas 20 extensões diferentes), o Counting Sort supera imensamente o Merge/Quick Sort, demonstrando que explorar a estrutura semântica dos dados permite escolhas algorítmicas muito mais precisas.
## Análise de Algoritmos para Datas (`SortCriteria::Data`)

A ordenação por data de modificação (timestamp `u64`) tem uma característica especial: **múltiplos arquivos podem compartilhar o mesmo timestamp** — por exemplo, arquivos extraídos de um `.zip` ou criados em lote por uma operação de sistema. Isso torna a **estabilidade** do algoritmo observável pelo usuário.

### Por que estabilidade importa aqui

Quando dois arquivos têm exatamente o mesmo timestamp, a ordem relativa entre eles é determinada pela posição original na listagem do diretório. Um algoritmo **estável** preserva essa ordem; um algoritmo **instável** pode invertê-la arbitrariamente.

Isso é o conceito de **ordenação multi-critério por composição estável**: se o usuário ordena por data e em seguida por nome *sem perder a ordem de data*, apenas um algoritmo estável entrega o resultado correto.

### Tabela comparativa

| Algoritmo      | Estável? | Comportamento com timestamps repetidos              |
|----------------|----------|-----------------------------------------------------|
| Merge Sort     | ✅        | Mantém ordem original entre arquivos com mesma data |
| Insertion Sort | ✅        | Idem — e eficiente se datas já quase-ordenadas      |
| Bubble Sort    | ✅        | Estável, mas O(n²) — apenas didático                |
| Quick Sort     | ❌        | Pode inverter a ordem de arquivos com mesma data    |
| Heap Sort      | ❌        | Idem — instável por design                          |

### Recomendação para `SortCriteria::Data`

**O Merge Sort é o algoritmo preferido para ordenação por data.**

Garante $O(n \log n)$ no pior caso, mantém estabilidade e é robusto para qualquer distribuição
de timestamps — incluindo o cenário onde 30%+ dos arquivos compartilham o mesmo timestamp.

**Hipótese verificada por benchmark** (`run_stability_date_benchmark`):
Com 30% de arquivos compartilhando o mesmo timestamp (interleaved), o Quick Sort pode inverter
a ordem relativa dentro do grupo de empate, enquanto o Merge Sort e o Insertion Sort preservam
a ordem original verificável por `verify_sort_stability`.
*/

pub mod common;
pub mod heap_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod counting_sort;

pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub use common::*;

pub use bubble_sort::BubbleSort;
pub use selection_sort::SelectionSort;
pub use insertion_sort::InsertionSort;
pub use counting_sort::CountingSort;
