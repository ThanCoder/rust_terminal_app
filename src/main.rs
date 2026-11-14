use std::io::stdin;

use crate::project_cache_cleaner::project_scanner::ProjectScanner;
mod project_cache_cleaner;

//Scan time: 1.59s
fn main() {
    let home = "/home/thancoder/projects";

    let mut s = ProjectScanner::new();
    // show_info(&mut s, PathBuf::from_str(home).unwrap());
    // show_size(&mut s);

    println!("Choose Number: ");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            println!("val: {}", input)
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }
}
