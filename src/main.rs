use std::path::Path;
use slint::{ModelRc, VecModel, ComponentHandle, Model};
use file_explorer_windows::core::file_metadata::{ClickResult, FileMetadata};

slint::include_modules!();

fn map_to_slint(files: Vec<FileMetadata>)-> ModelRc<FileInfo> {
    let slint_files: Vec<FileInfo> = files.into_iter().map(|f| {
        let is_dir = f.is_dir();
        let size_text = f.size_str(); 
        let date_text = f.modified_str();

        FileInfo {
            name: f.name.into(),      
            path: f.path.to_string_lossy().to_string().into(), 
            size: size_text.into(),   
            date: date_text.into(),
            is_directory: is_dir,     
        }
    }).collect();

    ModelRc::new(VecModel::from(slint_files))  
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    let root_path = "C:\\";
    let initial_files = FileMetadata::list_all_by_path(Path::new(root_path));

    ui.set_current_path(root_path.into());
    ui.set_files(map_to_slint(initial_files));

    // Quando o usuário clica em um arquivo na lista
    // 1. Apenas seleção (opcional, para atualizar status bar, etc)
    ui.on_file_selected({
        let ui_handle = ui_handle.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            ui.set_selected_file_index(index);
        }
    });

    // 2. Abertura real (Acontece apenas no Double Click)
    ui.on_file_double_click({
        let ui_handle = ui_handle.clone();
        move |index| {
            let ui = ui_handle.unwrap();
            if let Some(slint_file) = ui.get_files().row_data(index as usize) {
                let path_buf = std::path::PathBuf::from(slint_file.path.as_str());
                let node = FileMetadata::from_path(path_buf);

                match node.open() {
                    ClickResult::OpenedFolder(items) => {
                        ui.set_current_path(node.path.to_string_lossy().to_string().into());
                        ui.set_files(map_to_slint(items));
                        ui.set_selected_file_index(-1); 
                    }
                    ClickResult::OpenedFile => { /* Sistema abriu o arquivo */ }
                    ClickResult::Error(e) => { eprintln!("Error: {}", e) }
                }
            }
        }
    });

    // Quando o usuário clica no botão "Subir Nível" (Up)
    ui.on_navigate_up({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let current = std::path::PathBuf::from(ui.get_current_path().as_str());
            
            if let Some(parent) = current.parent() {
                let items = FileMetadata::list_all_by_path(parent);
                ui.set_current_path(parent.to_string_lossy().to_string().into());
                ui.set_files(map_to_slint(items));
            }
        }
    });

    ui.run()
}








// let mut start_path = String::from("C:\\Users");
// let mut list = FileMetadata::list_all_by_path(Path::new(&start_path));

// loop {
//     println!("PASTA ATUAL: {}", start_path);

//     for (i, item) in list.iter().enumerate() {
//         let prefixo = if item.is_dir() { "[DIR]" } else { "[FILE]" };
//         println!("{}: {} {}", i, prefixo, item.name);
//     }

//     print!("\nDigite o número para abrir (ou 'q' para sair): ");
//     io::stdout().flush().unwrap();

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let input = input.trim();

//     if input == "q" {break;}

//     if let Ok(idx) = input.parse::<usize>() {
//         if let Some(escolhido) = list.get(idx) {
//             match escolhido.open() {
//                 ClickResult::OpenedFolder(new_items) => {
//                     start_path = escolhido.path.to_string_lossy().to_string();
//                     println!("Entrando em...");
//                     list = new_items;
//                 },
//                 ClickResult::OpenedFile => println!("Arquivo aberto no Windows. Continuando na mesma pasta..."),
//                 ClickResult::Error(e) => eprintln!("Erro ao abrir: {}", e),
//             }
//         }
//     } else {
//         println!("Invalid Entry");
//     }
// }
