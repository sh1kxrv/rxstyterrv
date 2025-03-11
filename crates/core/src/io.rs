use std::{
  fs,
  path::{Path, PathBuf},
};

pub fn read_file(path: &PathBuf) -> String {
  let path = Path::new(path);
  if !path.exists() {
    panic!("file does not exist");
  }
  fs::read_to_string(path).unwrap()
}

pub fn write_file(path: &PathBuf, content: &str) {
  let path = Path::new(path);
  fs::write(path, content).unwrap();
}
