use std::{path::PathBuf, str::FromStr, time::Instant};

use terminal_rc::utils::{file_size_label, folder_size};

use crate::project_cache_cleaner::project_scanner::ProjectScanner;
mod project_cache_cleaner;

fn main() {
    let home = "/home/thancoder/projects";

    let mut s = ProjectScanner::new();
    show_info(&mut s, PathBuf::from_str(home).unwrap());
    show_size(&mut s);

    // println!("{}", file_size_label(512)); // 512.00 bytes
    // println!("{}", file_size_label(1024)); // 1.00 KB
    // println!("{}", file_size_label(1048576)); // 1.00 MB
    // println!("{}", file_size_label(1073741824)); // 1.00 GB
    // println!("{}", file_size_label(1099511627776)); // 1
}

fn show_info(s: &mut ProjectScanner, home_dir: PathBuf) {
    let start = Instant::now();
    s.scan(home_dir);

    let elapsed = start.elapsed();

    for p in &s.projects {
        println!("Project Name: {:?}", p.name());
        println!("Project Path: {:?}", p.home_dir);
        // println!("Project: {:?}", p);
    }

    println!("\nScan completed!");
    println!("Found projects: {}", s.projects.len());
    println!("Scan time: {:.2?}", elapsed);
}

fn show_size(s: &mut ProjectScanner) {
    let start = Instant::now();
    let mut total_size: u64 = 0;

    for p in &s.projects {
        for cd in &p.cache_folders {
            total_size += folder_size(&cd);
        }
    }

    let elapsed = start.elapsed();

    println!("\nScan completed!");
    println!("Found projects: {}", s.projects.len());
    println!("Cache Total Size: {}", file_size_label(total_size));
    println!("Scan time: {:.2?}", elapsed);
}
