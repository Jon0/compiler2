use std::env;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;


fn read_csv(path: &PathBuf) -> Vec<Vec<String>> {

    let mut buffer = String::new();

    let mut f = File::open(path).expect("file not found");

    f.read_to_string(&mut buffer);


    let mut lines: Vec<String> = buffer.split("\n")
        .map(|line| line.to_string())
        .collect();

    let mut parts = lines.into_iter()
        .map(|line| line.split(",")
            .map(|line| line.to_string())
            .collect())
        .collect();

    return parts;
}

fn get_link() {

    let mut testfile = PathBuf::new();

    match env::home_dir() {
        Some(path) => testfile = path,
        None => (),
    }
    testfile.push(".rust-pkg/data/test.csv");

    let data = read_csv(&testfile);


    println!("{:?}", data);
}


fn main() {
    get_link();
}
