use crate::core::file_metadata::FileMetadata;
use crate::sorting::{SortCriteria, Sorter, merge_sort::MergeSort};
use std::collections::{BTreeSet, HashMap};

pub struct CountingSort;

impl Sorter for CountingSort {
    fn sort(files: &mut Vec<FileMetadata>, criteria: SortCriteria) -> (usize, usize) {
        if criteria != SortCriteria::Tipo {
            // Fallback para MergeSort em caso de critérios com cardinalidade alta (Nome, Data, Tamanho)
            return MergeSort::sort(files, criteria);
        }

        let n = files.len();
        if n <= 1 {
            return (0, 0);
        }

        let mut comps = 0;
        let mut swaps = 0;

        // Passo 1: Como queremos ordenação secundária por nome dentro de cada tipo,
        // ordenamos tudo por nome usando MergeSort (que é estável).
        let (c_merge, s_merge) = MergeSort::sort(files, SortCriteria::Nome);
        comps += c_merge;
        swaps += s_merge;

        // Passo 2: Identificar todas as extensões únicas (para montar as categorias/buckets).
        // Usamos BTreeSet para que as extensões fiquem automaticamente em ordem alfabética.
        let mut unique_exts = BTreeSet::new();
        for f in files.iter() {
            if !f.is_dir() {
                unique_exts.insert(f.extension().to_lowercase());
            }
        }

        // Mapear cada extensão para um ID de categoria.
        // Categoria 0 é reservada para Diretórios.
        // Categoria 1..=k são para os arquivos, por extensão.
        let mut ext_to_category = HashMap::new();
        for (i, ext) in unique_exts.into_iter().enumerate() {
            ext_to_category.insert(ext, i + 1);
        }

        let k = ext_to_category.len();
        
        // Passo 3: Contagem das frequências
        let mut count = vec![0; k + 1];
        for f in files.iter() {
            let cat = if f.is_dir() {
                0
            } else {
                *ext_to_category.get(&f.extension().to_lowercase()).unwrap()
            };
            count[cat] += 1;
        }

        // Passo 4: Somas de prefixos (para encontrar a posição final de cada categoria)
        for i in 1..=k {
            count[i] += count[i - 1];
        }

        // Passo 5: Construir o array de saída (de trás para frente para manter estabilidade)
        // O Rust exige inicialização. Como vamos sobrescrever tudo, podemos apenas clonar o vetor original.
        let mut output = files.clone();
        for f in files.iter().rev() {
            let cat = if f.is_dir() {
                0
            } else {
                *ext_to_category.get(&f.extension().to_lowercase()).unwrap()
            };
            
            let idx = count[cat] - 1;
            output[idx] = f.clone();
            count[cat] -= 1;
            
            // Incrementamos a contagem de "trocas" / movimentações de memória.
            swaps += 1;
        }

        // Passo 6: Copiar de volta
        files.clone_from_slice(&output);
        swaps += n;

        // O número de comparações de chaves nesta fase é zero, pois usamos os buckets!
        // As únicas comparações foram as do BTreeSet (que podemos considerar ~ n log k).
        // Vamos aproximar as inserções do mapa e somar ao `comps`.
        comps += n; 

        (comps, swaps)
    }
}
