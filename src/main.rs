use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use slint::VecModel;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    // Stores the previous directories for Back navigation.
    let history: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));

    // Open a directory entered manually.
    let app_weak = app.as_weak();

    app.on_open_directory({
        let history = Rc::clone(&history);

        move |path| {
            let path = PathBuf::from(path.to_string());

            match open_folder(&path, &app_weak) {
                Ok(()) => {
                    history.borrow_mut().clear();

                    if let Some(app) = app_weak.upgrade() {
                        app.set_can_go_back(false);
                    }
                }
                Err(error) => {
                    eprintln!("Could not open directory: {error}");
                }
            }
        }
    });

    // Open a selected directory.
    let app_weak = app.as_weak();

    app.on_open_entry({
        let history = Rc::clone(&history);

        move |path| {
            let path = PathBuf::from(path.to_string());

            if let Some(app) = app_weak.upgrade() {
                let current_path = PathBuf::from(app.get_current_path().to_string());

                match open_folder(&path, &app_weak) {
                    Ok(()) => {
                        history.borrow_mut().push(current_path);

                        if let Some(app) = app_weak.upgrade() {
                            app.set_can_go_back(true);
                        }
                    }
                    Err(error) => {
                        eprintln!("Could not open directory: {error}");
                    }
                }
            }
        }
    });

    // Navigate backwards through directory history.
    let app_weak = app.as_weak();

    app.on_navigate_back({
        let history = Rc::clone(&history);

        move || {
            let previous_path = { history.borrow().last().cloned() };

            if let Some(path) = previous_path {
                match open_folder(&path, &app_weak) {
                    Ok(()) => {
                        history.borrow_mut().pop();

                        if let Some(app) = app_weak.upgrade() {
                            app.set_can_go_back(!history.borrow().is_empty());
                        }
                    }
                    Err(error) => {
                        eprintln!("Could not navigate back: {error}");
                    }
                }
            }
        }
    });

    // Navigate to the parent directory.
    let app_weak = app.as_weak();

    app.on_navigate_up({
        let history = Rc::clone(&history);

        move || {
            if let Some(app) = app_weak.upgrade() {
                let current_path = PathBuf::from(app.get_current_path().to_string());

                if let Some(parent_path) = current_path.parent() {
                    let parent_path = parent_path.to_path_buf();

                    match open_folder(&parent_path, &app_weak) {
                        Ok(()) => {
                            history.borrow_mut().push(current_path);

                            if let Some(app) = app_weak.upgrade() {
                                app.set_can_go_back(true);
                            }
                        }
                        Err(error) => {
                            eprintln!("Could not navigate up: {error}");
                        }
                    }
                }
            }
        }
    });

    // Navigate using a breadcrumb.
    let app_weak = app.as_weak();

    app.on_navigate_to({
        let history = Rc::clone(&history);

        move |path| {
            let path = PathBuf::from(path.to_string());

            if let Some(app) = app_weak.upgrade() {
                let current_path = PathBuf::from(app.get_current_path().to_string());

                if current_path == path {
                    return;
                }

                match open_folder(&path, &app_weak) {
                    Ok(()) => {
                        history.borrow_mut().push(current_path);

                        if let Some(app) = app_weak.upgrade() {
                            app.set_can_go_back(true);
                        }
                    }
                    Err(error) => {
                        eprintln!("Could not navigate to directory: {error}");
                    }
                }
            }
        }
    });

    // Select a file or directory.
    let app_weak = app.as_weak();

    app.on_select_entry(move |path| {
        if let Some(app) = app_weak.upgrade() {
            app.set_selected_path(path);
        }
    });

    let app_weak = app.as_weak();

    app.on_context_menu_action(move |action| {
        if let Some(app) = app_weak.upgrade() {
            let selected_path = app.get_selected_path().to_string();
            let action = action.to_string();

            match action.as_str() {
                "copy-path" => {
                    if selected_path.is_empty() {
                        println!("No file selected.");
                        return;
                    }

                    match arboard::Clipboard::new() {
                        Ok(mut clipboard) => match clipboard.set_text(selected_path.clone()) {
                            Ok(()) => {
                                println!("Path copied: {selected_path}");
                            }
                            Err(error) => {
                                eprintln!("Could not copy path: {error}");
                            }
                        },
                        Err(error) => {
                            eprintln!("Could not access clipboard: {error}");
                        }
                    }
                }

                "open" => {
                    println!("Open selected item: {selected_path}");
                }

                "rename" => {
                    println!("Rename selected item: {selected_path}");
                }

                "properties" => {
                    println!("Show properties: {selected_path}");
                }

                "delete" => {
                    println!("Delete selected item: {selected_path}");
                }

                _ => {}
            }
        }
    });

    app.run()
}

// Reads a directory and updates the Slint UI.
fn open_folder(path: &Path, app_weak: &slint::Weak<MainWindow>) -> std::io::Result<()> {
    let path = fs::canonicalize(path)?;
    let entries = fs::read_dir(&path)?;

    let mut files: Vec<FileEntry> = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!("Could not read directory entry: {error}");
                continue;
            }
        };

        let entry_path = entry.path();

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("Could not read metadata: {error}");
                continue;
            }
        };

        let is_directory = metadata.is_dir();

        let name = entry.file_name().to_string_lossy().into_owned();

        let extension = entry_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let icon = icon_for_entry(is_directory, &extension);

        let size = if is_directory {
            "-".to_string()
        } else {
            format_size(metadata.len())
        };

        let modified = metadata
            .modified()
            .ok()
            .map(|time| {
                chrono::DateTime::<chrono::Local>::from(time)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            })
            .unwrap_or_else(|| "-".to_string());

        files.push(FileEntry {
            name: name.into(),
            path: entry_path.to_string_lossy().to_string().into(),
            is_directory,
            size: size.into(),
            extension: extension.into(),
            icon: icon.into(),
            modified: modified.into(),
        });
    }

    // Sort directories first, followed by files.
    // Within each group, sort alphabetically.
    files.sort_by(|a, b| {
        b.is_directory.cmp(&a.is_directory).then_with(|| {
            a.name
                .to_string()
                .to_lowercase()
                .cmp(&b.name.to_string().to_lowercase())
        })
    });

    let model = Rc::new(VecModel::from(files));

    if let Some(app) = app_weak.upgrade() {
        app.set_files(model.into());

        app.set_current_path(path.to_string_lossy().to_string().into());

        app.set_selected_path("".into());

        update_breadcrumbs(&path, &app);

        app.set_can_go_up(path.parent().is_some());
    }

    Ok(())
}

// Builds the breadcrumb navigation from the current directory.
fn update_breadcrumbs(path: &Path, app: &MainWindow) {
    let mut breadcrumbs: Vec<Breadcrumb> = path
        .ancestors()
        .map(|ancestor| {
            let name = ancestor
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| ancestor.to_string_lossy().into_owned());

            Breadcrumb {
                name: name.into(),
                path: ancestor.to_string_lossy().to_string().into(),
            }
        })
        .collect();

    breadcrumbs.reverse();

    let model = Rc::new(VecModel::from(breadcrumbs));

    app.set_breadcrumbs(model.into());
}

// Converts file sizes into readable units.
fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{size:.1} {}", UNITS[unit_index])
}

// Returns an icon based on the file type.
fn icon_for_entry(is_directory: bool, extension: &str) -> &'static str {
    if is_directory {
        return "📁";
    }

    match extension {
        "pdf" => "📕",

        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => "🖼️",

        "mp4" | "mkv" | "mov" | "avi" => "🎬",

        "mp3" | "wav" | "flac" => "🎵",

        "doc" | "docx" | "odt" => "📝",

        "xls" | "xlsx" | "csv" => "📊",

        "zip" | "rar" | "7z" | "tar" | "gz" => "📦",

        "rs" | "py" | "js" | "ts" | "tsx" | "jsx" | "html" | "css" | "json" | "toml" => "💻",

        _ => "📄",
    }
}
