use std::io::prelude::*;
use std::fs::{DirEntry, File};
use std::collections::HashMap;
use search::{Error, State, Symbol, SearchAlgorithm};
use pprint;

pub struct KMP {
    pub pattern: String,
    pub dfa: HashMap<Symbol, Vec<State>>,
}

impl SearchAlgorithm for KMP {
    fn new(pat: String) -> KMP {
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
        KMP {
            pattern: pat,
            dfa: hmap,
        }
    }
    /// search fn uses Knuth-Morris-Pratt for searching pattern in files
    /// returns: Result index on success and Error on pattern mismatch
    fn search(&self, dir: DirEntry) -> Result<(), Error> {
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
            let mut pos_idx = 0;
            for (idx, chr) in line.char_indices() {
                if prev_state == total_state {
                    break;
                }
                prev_state = match self.dfa.get(&chr){
                    Some(val) => val[prev_state],
                    None => 0,
                };
                pos_idx = idx;
            }
            if prev_state == total_state {
                if pfname == true {
                    pprint::print_fname(dir.path().to_str().unwrap());
                    pfname = false;
                }
                let (strt, pat) = line.split_at(pos_idx - total_state + 1);
                let (pat, end) = pat.split_at(total_state);
                pprint::print_line(line_no+1, (strt, pat, end));
            }
        }
        Ok(())
    }
}
