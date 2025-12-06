mod traverse;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let root = Path::new(".");
    traverse::visit_dirs(root)?;
    Ok(())
}
