use std::{fs::read_dir, path::PathBuf};

use crate::project_cache_cleaner::{
    i_scanner::IScanner, project_info::ProjectInfo, scanners::FlutterScanner,
};

pub struct ProjectScanner {
    pub projects: Vec<ProjectInfo>,
    scanners: Vec<Box<dyn IScanner>>,
}

impl ProjectScanner {
    pub fn new() -> ProjectScanner {
        ProjectScanner {
            projects: vec![],
            scanners: vec![Box::new(FlutterScanner)],
        }
    }
    pub fn scan(&mut self, home_dir: PathBuf) {
        self.scan_project_type(home_dir);
        if self.scanners.is_empty() {
            return;
        }
        self.scan_project();
    }
    fn scan_project(&mut self) {
        for p in &self.projects {
            for scanner in &self.scanners {
                if p.p_type == scanner.project_type() {}
            }
        }
    }

    fn scan_project_type(&mut self, home_dir: PathBuf) {
        // scanner မရှိရင် ပေးမသွားဘူး
        if self.scanners.is_empty() {
            return;
        }
        self.projects.clear();
        let mut sub_folders: Vec<PathBuf> = vec![];
        sub_folders.push(home_dir);

        while !sub_folders.is_empty() {
            let dir = sub_folders.pop().unwrap();

            if dir.is_dir() {
                for e in read_dir(dir).unwrap() {
                    let p = e.unwrap().path();

                    if p.is_file() {
                        // check file
                        let name = p.file_name().unwrap();
                        for scanner in &self.scanners {
                            if scanner.is_match(&name.to_string_lossy().to_string()) {
                                let p = scanner.process(p.parent().unwrap().to_path_buf());
                                match p {
                                    None => {}
                                    Some(val) => {
                                        self.projects.push(val);
                                    }
                                }
                                continue;
                            }
                        }
                        // println!("path: {:?}", name);
                    } else if p.is_dir() {
                        sub_folders.push(p);
                    }
                }
            }
        }
    }
}
