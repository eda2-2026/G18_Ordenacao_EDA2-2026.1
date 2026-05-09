# Contexto do Projeto — G18 EDA2 2026.1

## Objetivo
Explorador de arquivos em Rust com Slint UI, usado como base para implementar e comparar algoritmos de **ordenação** e **busca** academicamente.

## Stack
- Rust (edition 2024)
- Slint 1.14.1 (UI declarativa, arquivos em `src/view/`)
- WalkDir, Rayon, DashMap, Serde, Chrono

## Estrutura-chave
- `src/core/file_metadata.rs` — modelo `FileMetadata`, listagem de arquivos, `ClickResult`
- `src/view/` — toda a UI Slint (frontend copiado de windows-fast-file-explorer)
- `src/main.rs` — wiring entre UI e lógica de backend
- `build.rs` — compila o Slint: `slint_build::compile("src/view/app-window.slint")`

## Algoritmos a implementar (futuro)
Os algoritmos de ordenação/busca devem ser adicionados em módulos separados (ex: `src/sorting/`, `src/search/`), sem modificar os arquivos Slint existentes.

## Origem do Frontend
Copiado de: https://github.com/Bappoz/windows-fast-file-explorer

## Colaboradores
2 pessoas — trabalho dividido via issues no GitHub
