use std::vec;

use macro_tuts::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    env: Vec<String>,
    args: Vec<String>,
}

fn main() {
    let command = Command::builder()
        .executable("/usr/bin/find")
        .env(vec![])
        .args(vec!["-c".to_owned(), "-v".to_owned()])
        .build()
        .unwrap();
    println!("{:#?}", command);
}
