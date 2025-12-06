use std::io;
use std::fs::read_dir;
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
                // ***** Set this to encryption call at some point*******
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}
 


