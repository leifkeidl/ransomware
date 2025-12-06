use std::{fs, io};
mod crypto;
mod traverse;
use std::path::Path;

fn main() -> io::Result<()> {

    // single file encryption test
    let file_path = "message.txt";
    crypto::encrypt(file_path);

    // directory traversal test
    let root = Path::new(".");
    traverse::visit_dirs(root)?;
    Ok(())
}
