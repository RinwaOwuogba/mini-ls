use std::{path::{Path, PathBuf}, error::Error, env::{self, Args}, io, fs};

pub fn run() {
  let path = get_config(&mut env::args());
  
  println!("Showing files in '{path}':");

  let path_reader = Box::new(FsPathReader::new(path));
  match get_files(path_reader) {
    Ok(files) => {
        for file_name in files {
            println!("- {file_name}")
        }
    },
    Err(e) => print!("Error getting files {}", e),
  };
}

fn get_files(path_reader: Box<dyn PathReader>) -> Result<Vec<String>, Box<dyn Error>> {
  let mut files = Vec::new();

  for entry in path_reader.read_dir()? {
    if let Ok(entry) = entry {
      files.push(entry);
    }
  }

  Ok(files)
} 

fn get_config(args: &mut Args) -> String {
  let _file_name = args.next();
  args.next().unwrap_or(get_current_dir())
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
    // let dummy_file_names = expected.iter().map(|f| String::from(*f)).collect();
    // let dummy_file_names: Vec<String> = expected.iter().map(|f| String::from(*f)).collect();

    let current_path_reader = Box::new(DummyPathReader::new(&expected));
    // let current_path_reader = Box::new(DummyPathReader::new(dummy_file_names));

    let got = match get_files(current_path_reader) {
        Ok(d) => d,
        Err(e) => panic!("Error while getting files {e}")
    };

    assert_array_equals(&expected, &got);
  }

  fn assert_array_equals(a: &Vec<&str>, b: &Vec<String>) {
      let b: Vec<&str> = b.into_iter().map(|f| f.as_str()).collect();

      assert_eq!(a.len(), b.len());

      for x in b.into_iter() {
          assert!(a.contains(&x), "\n{:?} does not contain '{}'\n", a, x);
      }
  }
}

type CustomReadDir = io::Result<Box<dyn Iterator<Item = Result<String, Box<dyn Error>>>>>;
trait PathReader {
  fn read_dir(&self) -> CustomReadDir;
}

struct FsPathReader {
  path: Box<PathBuf>
}

impl FsPathReader {
  fn new(path: String) -> Self {
    Self { path: Box::new(Path::new(&path).to_owned()) }
  }
}

impl PathReader for FsPathReader {
  fn read_dir(&self) -> CustomReadDir {
    match self.path.read_dir() {
      Ok(rd) => Ok(Box::new(ReadFsDir {
        read_dir: rd
      })),
      Err(e) => Err(e),
    }
  }
}

struct ReadFsDir {
  read_dir: fs::ReadDir
}

impl Iterator for ReadFsDir {
  type Item = Result<String, Box<dyn Error>>;

  fn next(&mut self) -> Option<Self::Item> {
      let option = self.read_dir.next();
      if option.is_none() {
        return Option::None
      }

      let entry = option.unwrap();
      match entry {
        Ok(entry) => Option::Some(Ok(String::from(entry.path().file_name().unwrap().to_str().unwrap()))),
        Err(e) => Option::Some(Err(Box::new(e)))
      }
    }
}

struct DummyPathReader {
  files: Vec<String>
}

impl DummyPathReader {
  fn new(file_names: &Vec<&str>) -> Self {
    // fn new(file_names: Vec<String>) -> Self {
    Self { files: file_names. }
  }
}

impl PathReader for DummyPathReader {
  fn read_dir(&self) -> CustomReadDir {
    Ok(
      Box::new(ReadDummyFs {
        read_dir: Box::new(self.files.clone().to_owned().into_iter())
      })
    )
  }
}

struct ReadDummyFs {
  read_dir: Box<dyn Iterator<Item = String>>
}

impl Iterator for ReadDummyFs {
  type Item = Result<String, Box<dyn Error>>;

  fn next(&mut self) -> Option<Self::Item> {
    let option =  self.read_dir.next();
    if option.is_none() {
      return Option::None;
    }

    let entry = option.unwrap();
    Option::Some(Ok(entry))
  }
}