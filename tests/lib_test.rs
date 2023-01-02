use std::{env, fs::{File, self}, path::{Path, PathBuf}};
use mini_ls::{self};

#[test]
fn it_reads_populated_dir_files() {
  let dir = env::temp_dir();
  let file_names = vec!["Cargo.toml", "src", ".git", "target", "Cargo.lock", ".gitignore"];
  let test_dir_path = Path::new(dir.to_str().unwrap()).join("test_dir");

  create_test_files(&file_names, &test_dir_path);
  let args = vec![String::from(""), String::from(test_dir_path.to_str().unwrap())];
  
  let (_path, files_read) = mini_ls::run(&args);
  mini_ls::assert_array_equals(&file_names, &files_read)
}

#[test]
fn it_reads_empty_dir_files() {
  let dir = env::temp_dir();
  let file_names = vec![];
  let test_dir_path = Path::new(dir.to_str().unwrap()).join("test_dir");

  create_test_files(&file_names, &test_dir_path);
  let args = vec![String::from(""), String::from(test_dir_path.to_str().unwrap())];
  
  let (_path, files_read) = mini_ls::run(&args);
  assert_eq!(files_read.len(), 0)
}

fn create_test_files(file_names: &Vec<&str>, test_dir_path: &PathBuf) {
  fs::remove_dir_all(test_dir_path).unwrap();

  fs::create_dir(test_dir_path).unwrap();

  for file_name in file_names {
    let file_path = test_dir_path.join(file_name);
    
    if let Err(e) = File::create(file_path) {
      panic!("Error creating test files {e}")
    }
  }
}