# G18 — File Explorer com Ordenação e Busca (EDA2 2026.1)

Projeto acadêmico do Grupo 18 para a disciplina de Estruturas de Dados e Algoritmos 2.

## Objetivo

Aplicar e comparar algoritmos de **ordenação** e **busca** em um explorador de arquivos real,
medindo desempenho e eficiência em cenários práticos.

## Stack

- **Linguagem:** Rust.
- **UI:** [Slint](https://slint.dev/) (frontend reutilizado do projeto [windows-fast-file-explorer](https://github.com/Bappoz/windows-fast-file-explorer))
- **Sistema de arquivos:** WalkDir + rayon (paralelismo)

## Estrutura

```
src/
├── main.rs               # Entrada da aplicação
├── lib.rs
├── core/
│   └── file_metadata.rs  # Modelo de dados de arquivo
├── view/                 # UI Slint (frontend)
│   ├── app-window.slint
│   ├── componets/
│   │   ├── file-item.slint
│   │   ├── file-list.slint
│   │   ├── sidebar.slint
│   │   ├── statusbar.slint
│   │   └── toolbar.slint
│   └── styles/
│       └── theme.slint
```

## Setup

```bash
cargo run
```

> Requer Rust 1.85+ (edition 2024)

## Grupo

| Membro | GitHub |
|--------|--------|
| ... | ... |
| ... | ... |
