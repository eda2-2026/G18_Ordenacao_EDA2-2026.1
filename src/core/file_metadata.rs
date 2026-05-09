#![allow(unused_imports)]

use std::time::SystemTime;
use chrono::{DateTime, Local};
use dashmap::mapref::entry;
use walkdir::{DirEntry, WalkDir};
use std::{fmt::format, fs::{File, Metadata}, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EntryType {
    File { size: u64, extension: String },
    Dir  { child_count: usize },
}

// Defini um resultado para um clique
pub enum ClickResult {
    OpenedFolder(Vec<FileMetadata>),
    OpenedFile,
    Error(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub path: PathBuf,
    kind: EntryType,
}

impl FileMetadata {

    pub fn from_path(path: PathBuf) -> Self {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let metadata = std::fs::metadata(&path);
        let is_dir = path.is_dir();

        let kind = if is_dir {
            EntryType::Dir { child_count: 0 }
        } else {
            let extension = path.extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let size = metadata.map(|m| m.len()).unwrap_or(0);
            EntryType::File { size, extension }
        };

        FileMetadata { name, path, kind }
    }

    // Contrutor do FileMetadata
    pub fn from_entry(entry: &DirEntry) -> Self {
        let path = entry.path().to_path_buf();

        let name = entry.file_name().to_string_lossy().to_string();
        let ft = entry.file_type();
        
        let kind = if ft.is_dir() {
            EntryType::Dir { child_count: 0 }
        } else {
            let extension = path.extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            
            // Pega o tamanho dos metadados que o WalkDir já tem
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            
            EntryType::File { size, extension }
        };

        FileMetadata { name, path, kind }
    }

    pub fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()   
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    // Check if is a Directory
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryType::Dir { .. })
    }

    // Check if is a file
    pub fn is_file(&self) -> bool {
        matches!(self.kind, EntryType::File { .. })
    }   

    // Clean the path and converts into PathBuf
    pub fn clean_path(input: &str) -> PathBuf {
        let cleaned = input.trim().trim_matches('"');
        PathBuf::from(cleaned)
    }

    pub fn list_all_by_path(path: &Path) -> Vec<FileMetadata> {

        WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| !FileMetadata::is_hidden(e))
            .filter_map(|res| res.ok())             
            .map(|entry| FileMetadata::from_entry(&entry))
            .collect()
    } 

    pub fn modified_str(&self) -> String{
        let metadata = std::fs::metadata(&self.path);
        if let Ok(m) = metadata {
            if let Ok(time) = m.modified() {
                let dt: DateTime<Local> = time.into();
                return dt.format("&d/%m/%Y %H:%M").to_string();
            }
        }
        "--/--/----".to_string()
    }


    pub fn open(&self) -> ClickResult {
        if self.is_dir() {
            let items = FileMetadata::list_all_by_path(&self.path);
            ClickResult::OpenedFolder(items)
        } else {
            match std::process::Command::new("explorer").arg(&self.path).spawn() {
                Ok(_) => ClickResult::OpenedFile,
                Err(e) => ClickResult::Error(e.to_string()),
            }
        }
    }    


    // Formatador de tamanho
    pub fn size_str(&self) -> String {
        match self.kind {
            EntryType::File { size,..} => {
                if size < 1024 { format!("{} B", size)}
                else if size < 1048576 { format!("{:.1} KB", size as f64 / 1024.0)}
                else {format!("{:.1} MB", size as f64 / 1048576.0)}
            },
            EntryType::Dir { .. } => String::from("<DIR>")
        }
    }
}