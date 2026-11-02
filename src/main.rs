use std::{path::PathBuf, str::FromStr, time::Instant};

use tabled::{Table, Tabled};
use terminal_rc::utils::file_size_label;

use crate::project_cache_cleaner::project_scanner::ProjectScanner;
mod project_cache_cleaner;

#[derive(Tabled)]
struct PTable {
    index: String,
    name: String,
    project_type: String,
    path: String,
    size: String,
}
//Scan time: 1.59s
fn main() {
    let home = "/home/thancoder/projects";

    let mut s = ProjectScanner::new();
    show_info(&mut s, PathBuf::from_str(home).unwrap());
    // show_size(&mut s);
}

fn show_info(s: &mut ProjectScanner, home_dir: PathBuf) {
    let start = Instant::now();
    // s.scan(home_dir);
    s.scan_with_thread(home_dir);

    let elapsed = start.elapsed();
    let mut data: Vec<PTable> = vec![];

    let mut i = 1;
    for p in &s.projects {
        // println!("Project Name: {:?}", p.name());
        // println!("Project Type: {:?}", p.p_type);
        // println!("Project Path: {:?}", p.home_dir);
        // println!("Project Size: {:?}", p.size_label());
        data.push(PTable {
            index: i.to_string(),
            name: p.name(),
            path: p.path_name(),
            project_type: p.p_type.as_string().to_string(),
            size: p.size_label(),
        });
        i += 1;
    }

    let tb = Table::new(data);
    print!("{tb}");

    let total = s.projects.iter().fold(0, |prev, e| {
        let mut all = prev;
        all += e.size;
        all
    });
    println!("\nScan completed!");
    println!("Found projects: {}", s.projects.len());
    println!("Projects Total Size: {}", file_size_label(total));
    println!("Scan time: {:.2?}", elapsed);
}
