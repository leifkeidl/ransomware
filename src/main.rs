use std::{fs, io};
mod crypto;
mod traverse;
use std::path::Path;

fn main() -> io::Result<()> {
    let root = Path::new("THEDIRPATH");
    traverse::visit_dirs(root)?;
    Ok(())
}
