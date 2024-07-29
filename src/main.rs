use std::{env, fs, io::Write};

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir = current_dir.as_os_str().to_str().unwrap();
    dbg!(current_dir);

    let directory_path = "./test_dir";

    let file_list = read_file_name(current_dir);
    
    write_file(&file_list);
}

fn read_file_name(directory_path: &str) -> Vec<String> {
    let entries = fs::read_dir(directory_path).expect("Failed to read directory.");
    let mut list = Vec::new();

    for entry in entries {
        match entry {
            Ok(file) => {
                let file_name = file.file_name().into_string().unwrap();
                list.push(file_name);
            }
            Err(err) => {
                let error = err;
                dbg!(error);
            }
        }
    }

    list.sort();
    list
}

fn write_file(file_list: &Vec<String>){
    let mut file = fs::File::create("file_names.txt").expect("Failed to create file.");
    for file_name in file_list{
        file.write_all(file_name.as_bytes()).expect("Failed to write file.");
        file.write_all("\n".as_bytes()).expect("Failed to write new line.");
    }
}

fn print_file_list(file_list: &Vec<String>){
    for file in file_list{
        println!("{}", file);
    }
}