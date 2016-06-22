// TODO: Output format in :
// <filename>
// <line_number>: text
// .
// .
// <filename>
use std::io;
use std::path::Path;
use std::fs::{self, DirEntry};
use std::collections::HashMap;


type Symbol = char;
type State = usize;

pub enum Error {
    PatternNotFound,
}

#[derive(Debug)]
struct Pattern {
    pattern: &'static str,
    dfa: HashMap<Symbol, Vec<State>>,
}

impl Pattern {
    pub fn new(pat: &'static str) -> Pattern {
        let chars = pat.chars();
        let n_chars = chars.count();
        let mut hmap: HashMap<Symbol, Vec<State>> = HashMap::new();
        for ch in pat.chars() {
            hmap.clone().get(&ch).ok_or_else(|| {
                hmap.insert(ch, vec![0; n_chars]);
            });
        }
        Pattern {
            pattern: pat,
            dfa: hmap,
        }
    }
    /// search fn uses Knuth-Morris-Pratt for searching pattern in files
    /// returns: Result index on success and Error on pattern mismatch
    pub fn search(dir: DirEntry) -> Result<usize, Error>{
        // Knuth Morris Pratt :)
        Ok(6)
    }
}

/// This function recursively walks the dir path and applies search fn
/// on every entry which is file and not directory
fn visit_dirs(dir: &Path, search: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), search));
            } else {
                println!("{}", entry.path().display());
            }
        }
    }
    Ok(())
}

fn main() {
    let path = Path::new(".");
    
}
