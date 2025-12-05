use std::fs;
mod crypto;

fn main() {

let file_path = "message.txt";

crypto::encrypt(file_path);

}
