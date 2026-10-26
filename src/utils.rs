use std::path::PathBuf;

pub fn folder_size(dir: &PathBuf) -> u64 {
    let mut total_size: u64 = 0;
    if dir.is_dir() {
        if let Ok(entry) = dir.read_dir() {
            for e in entry.flatten() {
                let p = e.path();
                if p.is_file() {
                    if let Ok(meta) = p.metadata() {
                        total_size += meta.len();
                    }
                } else if p.is_dir() {
                    total_size += folder_size(&p);
                }
            }
        }
        return total_size;
    }
    0
}

pub fn file_size_label(size: u64) -> String {
    let mut size = size as f64;
    let mut i = 0;
    let pow = 1024.0;
    let lables = vec!["bytes", "KB", "MB", "GB", "TB"];

    while size >= pow && i < lables.len() - 1 {
        size /= pow;
        i += 1;
    }
    format!("{:.2} {}", size, lables[i])
}
