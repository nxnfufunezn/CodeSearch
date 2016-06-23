use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::{self, DirEntry, File};
use std::collections::HashMap;
use std::str;
use pprint;
use results::InFileSearch;

type Symbol = char;
type State = usize;

pub enum Error {
    PatternNotFound,
    FileNotReadable(String),
    NotAbleToOpenFile(String),
}

#[derive(Debug)]
pub struct Pattern {
    pattern: String,
    dfa: HashMap<Symbol, Vec<State>>,
}

impl Pattern {
    pub fn new(pat: String) -> Pattern {
        let n_chars = pat.chars().count();
        let mut hmap: HashMap<Symbol, Vec<State>> = HashMap::new();
        for ch in pat.chars() {
            hmap.entry(ch).or_insert(vec![0; n_chars]);
        }
        if let Some(idx) = hmap.get_mut(&pat.chars().nth(0).unwrap()) {
            idx[0] = 1;
        }
        let mut prev_match = 0;
        for j in 1..n_chars {
            for (_, v_idx) in hmap.iter_mut() {
                v_idx[j] = v_idx[prev_match];
            }
            if let Some(v_idx) = hmap.get_mut(&pat.chars().nth(j).unwrap()) {
                v_idx[j] = j + 1;
            }
            prev_match = hmap.get(&pat.chars()
                    .nth(j)
                    .unwrap())
                .unwrap()[prev_match];
        }
        Pattern {
            pattern: pat,
            dfa: hmap,
        }
    }
    /// search fn uses Knuth-Morris-Pratt for searching pattern in files
    /// returns: Result index on success and Error on pattern mismatch
    pub fn search(&mut self, dir: &DirEntry) -> Result<(), Error> {
        let mut buffer = String::new();
        File::open(dir.path())
            .map_err(|err| Error::NotAbleToOpenFile(err.to_string()))
            .and_then(|mut file| {
                file.read_to_string(&mut buffer)
                    .map_err(|err| Error::FileNotReadable(err.to_string()))
            });
        let mut pfname = true;
        let total_state = self.pattern.chars().count();
        for (line_no, line) in buffer.lines().enumerate() {
            let mut prev_state = 0;
            for chr in line.chars() {
                if prev_state == total_state {
                    break;
                }
                prev_state = match self.dfa.get(&chr){
                    Some(val) => val[prev_state],
                    None => 0,
                };
            }
            if prev_state == total_state {
                if pfname == true {
                    pprint::print_fname(dir.path().to_str().unwrap());
                    pfname = false;
                }
                pprint::print_line(line_no, line);
            }
        }
        Ok(())
    }

    /// This function recursively walks the dir path and applies search fn
    /// on every entry which is file and not directory
    pub fn recursive_search(&mut self, dir: &Path) -> io::Result<()> {
        if try!(fs::metadata(dir)).is_dir() {
            for entry in try!(fs::read_dir(dir)) {
                let entry = try!(entry);
                if try!(fs::metadata(entry.path())).is_dir() {
                    try!(self.recursive_search(&entry.path()));
                } else {
                    self.search(&entry);
                }
            }
        }
        Ok(())
    }
}
