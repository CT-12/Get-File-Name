use core::panic;
use std::{env, fs, io::Write};

struct Config{
    dir_path: String,
}

impl Config {
    fn new(path: String) -> Config {
        Config {
            dir_path: path,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(&args);

    println!("Read path: {}", path);

    let config = Config::new(path);
    let file_list = read_file_name(&config);
    write_file(&file_list);
}

fn parse_args(args: &Vec<String>) -> String {
    let path: String;

    if args.len() > 2{
        panic!("Error: Only one parameter is allowed !");
    } else if args.len() == 2{
        path = args[1].clone();
    } else {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let current_dir = current_dir.as_os_str().to_str().unwrap().to_string();
        path = current_dir
    }

    path
}

fn read_file_name(config: &Config) -> Vec<String> {
    let directory_path = &config.dir_path;
    let entries = fs::read_dir(directory_path).expect("Failed to read directory.");
    let mut list = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();
                list.push(file_name);
        }
    }

    list.sort();
    list
}

fn write_file(file_list: &Vec<String>) {
    let mut file = fs::File::create("file_names.txt").expect("Failed to create file.");
    for file_name in file_list {
        file.write_all(file_name.as_bytes())
            .expect("Failed to write file.");
        file.write_all("\n".as_bytes())
            .expect("Failed to write new line.");
    }
}