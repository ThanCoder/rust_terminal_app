use std::path::PathBuf;

use terminal_rc::utils::file_size_label;

use crate::project_cache_cleaner::project_type::ProjectType;

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub p_type: ProjectType,
    pub home_dir: PathBuf,
    pub cache_folders: Vec<PathBuf>,
    pub size: u64,
}

impl ProjectInfo {
    pub fn path(&self) -> String {
        self.home_dir.to_str().unwrap().to_string()
    }
    pub fn path_name(&self) -> String {
        self.home_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
    pub fn size_label(&self) -> String {
        file_size_label(self.size)
    }
    pub fn name(&self) -> String {
        self.home_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
}
