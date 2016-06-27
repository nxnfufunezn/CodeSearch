use std::fs::{self, DirEntry};
use std::path::Path;
use crossbeam;
use std::marker::Sync;

pub type Symbol = char;
pub type State = usize;

#[derive(Debug)]
pub enum Error {
    FileNotReadable(String),
    NotAbleToOpenFile(String),
}

pub trait SearchAlgorithm {
    fn new(pat: String) -> Self;
    fn search(&self, DirEntry) -> Result<(), Error>;

    /// Recursively walk the dir path and apply search fn
    /// on every entry which is file and not directory
    fn recursive_search(&self, dir: &Path) where Self: Sync {
        if fs::metadata(dir).unwrap().is_dir() {
            crossbeam::scope(|scope| {
                for entry in fs::read_dir(dir).unwrap() {
                    let entry = entry.unwrap();
                    if fs::metadata(entry.path()).unwrap().is_dir() {
                        self.recursive_search(&entry.path());
                    } else {
                        scope.spawn(move || {
                            self.search(entry).unwrap();
                        });
                    }
                }
                
            });
        }
    }
}
