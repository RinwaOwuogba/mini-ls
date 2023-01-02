use std::{path::{Path}, error::Error, env, fs, str::FromStr};

pub fn run(args: &Vec<String>) -> (String, Vec<String>) {
  let path = get_config(args);

  let mut path_iter = ReadFsDir::new(&path);
  (path, get_files(&mut path_iter))
}

fn get_files<'a,>(path_reader: &'a mut dyn Iterator<Item = FsIterItem>) -> Vec<String> {
    let mut files = Vec::new();

    for entry in path_reader {
      if let Ok(entry) = entry {
        files.push(entry);
      }
    }
    
    files
} 

fn get_config(args: &Vec<String>) -> String {
  if args.len() < 2 { return get_current_dir().to_string() }

  String::from(&args[1])
}

fn get_current_dir() -> String {
  String::from(env::current_dir().unwrap().to_str().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_files() {
    let expected = vec!["Cargo.toml", "src", ".git", "target", "Cargo.lock", ".gitignore"];

    let mut path_iter = ReadDummyFs::new(&expected);
    let got = get_files(&mut path_iter);

    assert_array_equals(&expected, &got);
  }

  struct ReadDummyFs<'a> {
    read_dir: Box<dyn Iterator<Item = &'a &'a str> + 'a>
  }
  
  impl<'a> ReadDummyFs<'a> {
    fn new(file_names: &'a Vec<&'a str>) -> Self {
      Self { read_dir: Box::new(file_names.iter())}
    }
  }
  
  impl<'a> Iterator for ReadDummyFs<'a> {
    type Item = FsIterItem;
  
    fn next(&mut self) -> Option<Self::Item> {
      let option =  self.read_dir.next();
      if option.is_none() {
        return Option::None;
      }
  
      let entry = *option.unwrap();
      Option::Some(Ok(String::from_str(entry).unwrap()))
    }
  }
}

type FsIterItem = Result<String, Box<dyn Error>>;

struct ReadFsDir {
  read_dir: fs::ReadDir
}

impl ReadFsDir {
  fn new(path: &str) -> Self {
    Self{ read_dir: Path::new(path).read_dir().unwrap(), }
  }
}

impl Iterator for ReadFsDir {
  type Item = FsIterItem;

  fn next(&mut self) -> Option<Self::Item> {
      let option = self.read_dir.next();
      if option.is_none() {
        return Option::None
      }

      let entry = option.unwrap();
      match entry {
        Ok(entry) => Option::Some(Ok(String::from_str(entry.path().file_name().unwrap().to_str().unwrap()).unwrap())),
        Err(e) => Option::Some(Err(Box::new(e)))
      }
    }
}

pub fn assert_array_equals(a: &Vec<&str>, b: &Vec<String>) {
  let b: Vec<&str> = b.into_iter().map(|f| f.as_str()).collect();

  assert_eq!(a.len(), b.len());

  for x in b.into_iter() {
      assert!(a.contains(&x), "\n{:?} does not contain '{}'\n", a, x);
  }
}