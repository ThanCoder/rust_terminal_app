use std::{
    fs::read_dir,
    path::PathBuf,
    sync::{Arc, Mutex, mpsc},
    thread,
};

use terminal_rc::utils::folder_size;

use crate::project_cache_cleaner::{
    i_scanner::IScanner, project_info::ProjectInfo, scanners::{FlutterScanner, RustScanner},
};

pub struct ProjectScanner {
    pub projects: Vec<ProjectInfo>,
    scanners: Vec<Box<dyn IScanner>>,
}

impl ProjectScanner {
    pub fn new() -> ProjectScanner {
        ProjectScanner {
            projects: vec![],
            scanners: vec![Box::new(FlutterScanner), Box::new(RustScanner)],
        }
    }

    pub fn scan(&mut self, home_dir: PathBuf) {
        self.scan_project_type(home_dir);
    }
    pub fn scan_with_thread(&mut self, home_dir: PathBuf) {
        self.scan_project_type_with_thread(home_dir);
    }

    fn scan_project_type_with_thread(&mut self, home_dir: PathBuf) {
        if self.scanners.is_empty() {
            return;
        }
        let worker_count = 4;
        self.projects.clear();

        let (dir_tx, dir_rx) = mpsc::channel::<PathBuf>();
        let (pro_tx, pro_rx) = mpsc::channel::<ProjectInfo>();

        let scanners = &self.scanners;

        thread::scope(|s| {
            // scan dir
            let dir_worker = s.spawn(move || {
                let mut queue = vec![home_dir];
                while let Some(dir) = queue.pop() {
                    if let Ok(ents) = read_dir(dir) {
                        for ent in ents.flatten() {
                            let path = ent.path();
                            if path.is_file() {
                                if dir_tx.send(path).is_err() {
                                    break;
                                }
                            } else if path.is_dir() {
                                queue.push(path);
                            }
                        }
                    } else {
                        continue;
                    }
                }
            });
            // scan pro

            let mut workers = vec![];
            let dir_rx = Arc::new(Mutex::new(dir_rx));

            for _ in 0..worker_count {
                let rx = Arc::clone(&dir_rx);
                let tx = pro_tx.clone();

                let wk = s.spawn(move || {
                    loop {
                        let task = {
                            let rc = rx.lock().unwrap();
                            rc.recv()
                        };
                        if let Ok(path) = task {
                            // println!("Worker {id}: {:?}", path);
                            let name = path.file_name().unwrap();
                            for scanner in scanners {
                                if scanner.is_match(&name.to_string_lossy().to_string()) {
                                    let p = scanner.process(path.parent().unwrap().to_path_buf());
                                    match p {
                                        None => {
                                            break;
                                        }
                                        Some(val) => {
                                            let mut pro = val;
                                            pro.size = folder_size(&pro.home_dir);
                                            // self.projects.push(val);
                                            if tx.send(pro).is_err() {
                                                break;
                                            }
                                        }
                                    }

                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                });

                workers.push(wk);
            }

            dir_worker.join().unwrap();
            drop(pro_tx);

            for pw in workers {
                pw.join().unwrap();
            }
        });

        // main thread
        self.projects = pro_rx.into_iter().collect();
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
                                        let mut pro = val;
                                        pro.size = folder_size(&pro.home_dir);
                                        self.projects.push(pro);
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
