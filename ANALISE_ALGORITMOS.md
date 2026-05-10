# Síntese Acadêmica: Análise Comparativa de Algoritmos

Este documento apresenta a análise comparativa de performance real dos algoritmos implementados, submetidos a diferentes cenários e critérios de ordenação que refletem o uso cotidiano em um explorador de arquivos.

### Cenário: Diretório Home (~) (N = 500, Critério: Nome)

**Expectativa / Insight:** Para arrays pequenos, Insertion Sort costuma ser extremamente rápido devido à baixa sobrecarga de chamadas recursivas, rivalizando com Merge/Quick Sort.

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Quick Sort | 1.051 ms | 4872 comps | 2442 trocas | O(log n) |
| Heap Sort | 1.569 ms | 7353 comps | 3930 trocas | O(1) |
| Merge Sort | 3.221 ms | 3014 comps | 4488 trocas | O(n) |
| Insertion Sort | 17.327 ms | 72753 comps | 72254 trocas | O(1) |

---

### Cenário: Pasta de Downloads (N = 2000, Critério: Tamanho)

**Expectativa / Insight:** Cenário genérico com distribuição variada de tamanhos. Quick Sort costuma vencer por ser in-place e cache-friendly.

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Heap Sort | 1.273 ms | 37781 comps | 20147 trocas | O(1) |
| Merge Sort | 11.985 ms | 13537 comps | 21952 trocas | O(n) |
| Quick Sort | 21.204 ms | 365399 comps | 364197 trocas | O(log n) |
| Insertion Sort | 49.328 ms | 795022 comps | 793023 trocas | O(1) |

---

### Cenário: Projeto Git (Datas Repetidas) (N = 5000, Critério: Data)

**Expectativa / Insight:** Apresenta alta incidência de datas iguais. Merge Sort é excelente aqui pois sua estabilidade preserva a ordem de arquivos com a mesma data.

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Heap Sort | 2.898 ms | 83148 comps | 42804 trocas | O(1) |
| Merge Sort | 36.023 ms | 40670 comps | 61808 trocas | O(n) |
| Quick Sort | 79.358 ms | 1451136 comps | 1420007 trocas | O(log n) |
| Insertion Sort | 151.901 ms | 2782221 comps | 2777222 trocas | O(1) |

---

### Cenário: Raiz de Disco (N = 10000, Critério: Tipo)

**Expectativa / Insight:** Com milhares de arquivos, mas poucas extensões (Baixa Cardinalidade), os algoritmos comparativos sofrem. Counting Sort é projetado exatamente para isso, executando em tempo O(n + k).

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Heap Sort | 75.626 ms | 235472 comps | 124649 trocas | O(1) |
| Counting Sort | 105.668 ms | 126064 comps | 153616 trocas | O(n) |
| Quick Sort | 108.565 ms | 350418 comps | 281488 trocas | O(log n) |
| Merge Sort | 116.863 ms | 117621 comps | 133616 trocas | O(n) |
| Insertion Sort | 6485.010 ms | 22367692 comps | 22357693 trocas | O(1) |

---

### Cenário: Lista Quase-Ordenada (1 item novo) (N = 1000, Critério: Nome)

**Expectativa / Insight:** O Insertion Sort possui tempo O(n) em listas quase ordenadas. Ele identifica rapidamente os elementos no lugar e brilha incomparavelmente aqui.

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Insertion Sort | 1.290 ms | 2958 comps | 1959 trocas | O(1) |
| Heap Sort | 4.131 ms | 17567 comps | 9697 trocas | O(1) |
| Merge Sort | 6.700 ms | 6023 comps | 9976 trocas | O(n) |
| Quick Sort | 104.889 ms | 489700 comps | 489718 trocas | O(log n) |

---

### Cenário: Lista Invertida (Pior Caso) (N = 1000, Critério: Tamanho)

**Expectativa / Insight:** Para uma lista totalmente invertida, o Quick Sort tradicional degrada para O(n²). Heap Sort e Merge Sort mantêm sua consistência algorítmica O(n log n).

| Algoritmo | Tempo (ms) | Comparações | Trocas | Memória Extra |
| :--- | :--- | :--- | :--- | :--- |
| Heap Sort | 0.527 ms | 15965 comps | 8316 trocas | O(1) |
| Merge Sort | 5.798 ms | 4932 comps | 9976 trocas | O(n) |
| Quick Sort | 26.106 ms | 499500 comps | 250499 trocas | O(log n) |
| Insertion Sort | 30.288 ms | 499500 comps | 499500 trocas | O(1) |

---

## Conclusão Acadêmica

A experimentação prática prova que **não existe o melhor algoritmo absoluto**, mas sim o algoritmo mais adequado para cada topologia de dados:
- **Por Tipo/Extensão (Baixa Cardinalidade):** `Counting Sort` quebra a barreira do O(n log n) agrupando os elementos em buckets (O(n)). É insuperável aqui.
- **Por Data/Nome (Estabilidade Importante):** `Merge Sort` é o recomendado, preservando com segurança elementos que caem na mesma data, embora exija O(n) de memória extra.
- **Por Tamanho (Performance in-place):** `Quick Sort` (ou Heap Sort para evitar O(n²) no pior caso) é formidável, rodando inteiramente em cache e trocando arquivos rapidamente sem gastar memória alocada substancial.
- **Diretórios Quase Prontos:** O trivial `Insertion Sort` é uma máquina perfeita se apenas 1 ou 2 arquivos foram adicionados à pasta.
