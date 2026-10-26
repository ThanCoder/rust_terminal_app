use std::path::PathBuf;

use crate::project_cache_cleaner::{project_info::ProjectInfo, project_type::ProjectType};

pub trait IScanner {
    fn project_type(&self) -> ProjectType;
    fn cache_folder_names(&self) -> Vec<String>;
    fn is_match(&self, filename: &String) -> bool;
    fn should_queue_sub_folder(&self, dirname: &str) -> bool;
    fn process(&self, current_dir: PathBuf) -> Option<ProjectInfo> {
        // println!("P: {:?}", self.project_type());
        // println!(
        //     "current_dir: {:?} - is_dir: {}",
        //     current_dir,
        //     current_dir.is_dir()
        // );
        let mut cache_folders: Vec<PathBuf> = vec![];

        let mut subs: Vec<PathBuf> = vec![];
        subs.push(current_dir.clone());
        while !subs.is_empty() {
            let dir = subs.pop().unwrap();
            // println!("dir: {:?} , is_dir: {}", dir,dir.is_dir());

            if !dir.is_dir() {
                continue;
            }

            for en in dir.read_dir().unwrap() {
                let e = en.unwrap();
                let path = e.path().to_path_buf();

                if path.is_dir() {
                    let dirname = e.file_name().to_string_lossy().to_string();
                    // add cache folder
                    if self.cache_folder_names().contains(&dirname) {
                        cache_folders.push(path.clone());
                    }
                    // add sub folder
                    if self.should_queue_sub_folder(&dirname) {
                        subs.push(path);
                    }
                }
            }
        }
        if !cache_folders.is_empty() {
            let p = ProjectInfo {
                home_dir: current_dir,
                p_type: self.project_type(),
                cache_folders,
            };
            return Some(p);
        }

        None
    }
}
