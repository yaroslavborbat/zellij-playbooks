use crate::tab_manager::TabManager;
use std::fs;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub(crate) struct FileItem {
    pub id: usize,
    pub name: String,
}

impl FileItem {
    fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct FilePicker {
    items: Vec<FileItem>,
}

impl FilePicker {
    pub(crate) fn load_files(
        &mut self,
        cwd: &PathBuf,
        sort: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut files = Vec::new();

        if let Ok(entries) = fs::read_dir(cwd) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                // Only include regular files, skip directories and hidden files
                if path.is_file()
                    && !path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.starts_with('.'))
                        .unwrap_or(false)
                {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    files.push(name)
                }
            }
        }

        if sort {
            files.sort();
        }

        let mut items = Vec::new();
        for (i, file) in files.iter().enumerate() {
            let item = FileItem::new(i + 1, file.clone());
            items.push(item);
        }

        self.items = items;

        Ok(())
    }

    pub(crate) fn manager(&self) -> TabManager<FileItem> {
        TabManager::new(self.items.clone())
    }
}
