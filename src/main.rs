use file_explorer_eda2::core::file_metadata::{ClickResult, FileMetadata};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

slint::include_modules!();

fn map_to_slint(files: Vec<FileMetadata>) -> ModelRc<FileInfo> {
    let slint_files: Vec<FileInfo> = files
        .into_iter()
        .map(|f: FileMetadata| {
            let is_dir = f.is_dir();
            let size = f.size_str();
            let date = f.modified_str();
            FileInfo {
                name: f.name.into(),
                path: f.path.to_string_lossy().to_string().into(),
                size: size.into(),
                date: date.into(),
                is_directory: is_dir,
            }
        })
        .collect();
    ModelRc::new(VecModel::from(slint_files))
}

fn do_navigate(ui: &MainWindow, path: &Path) {
    let items = FileMetadata::list_all_by_path(path);
    ui.set_current_path(path.to_string_lossy().to_string().into());
    ui.set_files(map_to_slint(items));
    ui.set_selected_file_index(-1);
}

struct History {
    entries: Vec<PathBuf>,
    pos: usize,
}

impl History {
    fn new(initial: PathBuf) -> Self {
        Self { entries: vec![initial], pos: 0 }
    }

    fn push(&mut self, path: PathBuf) {
        self.entries.truncate(self.pos + 1);
        self.entries.push(path);
        self.pos = self.entries.len() - 1;
    }

    fn go_back(&mut self) -> Option<PathBuf> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(self.entries[self.pos].clone())
        } else {
            None
        }
    }

    fn go_forward(&mut self) -> Option<PathBuf> {
        if self.pos + 1 < self.entries.len() {
            self.pos += 1;
            Some(self.entries[self.pos].clone())
        } else {
            None
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| if cfg!(windows) { "C:\\".to_string() } else { "/".to_string() });
    let start_path = PathBuf::from(&home);

    do_navigate(&ui, &start_path);

    let raiz_path = if cfg!(windows) { "C:\\" } else { "/" };

    // Locations com paths reais
    let locs: Vec<Location> = vec![
        Location { name: "Home".into(), path: home.clone().into(), icon: "🏠".into() },
        Location {
            name: "Desktop".into(),
            path: Path::new(&home).join("Desktop").to_string_lossy().to_string().into(),
            icon: "🖥️".into(),
        },
        Location {
            name: "Documentos".into(),
            path: Path::new(&home).join("Documents").to_string_lossy().to_string().into(),
            icon: "📄".into(),
        },
        Location {
            name: "Downloads".into(),
            path: Path::new(&home).join("Downloads").to_string_lossy().to_string().into(),
            icon: "⬇️".into(),
        },
        Location {
            name: "Imagens".into(),
            path: Path::new(&home).join("Pictures").to_string_lossy().to_string().into(),
            icon: "🖼️".into(),
        },
        Location { name: "Raiz".into(), path: raiz_path.into(), icon: "💾".into() },
    ];
    ui.set_locations(ModelRc::new(VecModel::from(locs)));

    let history = Rc::new(RefCell::new(History::new(start_path)));

    ui.on_file_selected({
        let ui_handle = ui_handle.clone();
        move |index| {
            ui_handle.unwrap().set_selected_file_index(index);
        }
    });

    ui.on_file_double_click({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            if let Some(slint_file) = ui.get_files().row_data(index as usize) {
                let path_buf = PathBuf::from(slint_file.path.as_str());
                let node = FileMetadata::from_path(path_buf.clone());

                match node.open() {
                    ClickResult::OpenedFolder(items) => {
                        history.borrow_mut().push(path_buf.clone());
                        ui.set_current_path(
                            path_buf.to_string_lossy().to_string().into(),
                        );
                        ui.set_files(map_to_slint(items));
                        ui.set_selected_file_index(-1);
                        ui.set_selected_location_index(-1);
                    }
                    ClickResult::OpenedFile => {}
                    ClickResult::Error(e) => eprintln!("Error: {}", e),
                }
            }
        }
    });

    ui.on_navigate_up({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move || {
            let ui = ui_handle.unwrap();
            let current = PathBuf::from(ui.get_current_path().as_str());
            if let Some(parent) = current.parent() {
                let parent = parent.to_path_buf();
                history.borrow_mut().push(parent.clone());
                do_navigate(&ui, &parent);
                ui.set_selected_location_index(-1);
            }
        }
    });

    ui.on_navigate_back({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move || {
            let ui = ui_handle.unwrap();
            if let Some(path) = history.borrow_mut().go_back() {
                do_navigate(&ui, &path);
                ui.set_selected_location_index(-1);
            }
        }
    });

    ui.on_navigate_forward({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move || {
            let ui = ui_handle.unwrap();
            if let Some(path) = history.borrow_mut().go_forward() {
                do_navigate(&ui, &path);
                ui.set_selected_location_index(-1);
            }
        }
    });

    // Disparado ao pressionar Enter na barra de endereço
    ui.on_navigate_to({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move |path_str| {
            let path = PathBuf::from(path_str.as_str());
            if path.is_dir() {
                let ui = ui_handle.unwrap();
                history.borrow_mut().push(path.clone());
                do_navigate(&ui, &path);
                ui.set_selected_location_index(-1);
            }
        }
    });

    ui.on_refresh_files({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let current = PathBuf::from(ui.get_current_path().as_str());
            let items = FileMetadata::list_all_by_path(&current);
            ui.set_files(map_to_slint(items));
            ui.set_selected_file_index(-1);
        }
    });

    ui.on_location_selected({
        let ui_handle = ui_handle.clone();
        let history = history.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            if let Some(loc) = ui.get_locations().row_data(index as usize) {
                let path = PathBuf::from(loc.path.as_str());
                if path.is_dir() {
                    history.borrow_mut().push(path.clone());
                    do_navigate(&ui, &path);
                    ui.set_selected_location_index(index);
                }
            }
        }
    });

    ui.run()
}
