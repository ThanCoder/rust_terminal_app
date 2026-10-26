use std::path::PathBuf;

use crate::project_cache_cleaner::project_type::ProjectType;

#[derive(Debug)]
pub struct ProjectInfo {
    pub p_type: ProjectType,
    pub home_dir: PathBuf,
    pub cache_folders: Vec<PathBuf>,
}

impl ProjectInfo {
    pub fn name(&self) -> String {
        self.home_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
}
