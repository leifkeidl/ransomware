use std::fs;


pub fn read_dir(){
    let paths = fs::read_dir("/home/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
