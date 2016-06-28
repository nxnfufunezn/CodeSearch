use std::io::prelude::*;
use std::fs::{DirEntry, File};
use search::{Error, SearchAlgorithm};
use pprint;

pub struct LIB {
    pattern: String,
}

impl SearchAlgorithm for LIB {
    fn new(pat: String) -> Self {
        LIB {
            pattern: pat,
        }
    }

    fn search(&self, dir: DirEntry) -> Result<(), Error> {
        let mut buffer = String::new();
        File::open(dir.path())
            .map_err(|err| Error::NotAbleToOpenFile(err.to_string()))
            .and_then(|mut file| {
                file.read_to_string(&mut buffer)
                    .map_err(|err| Error::FileNotReadable(err.to_string()))
            });
        let mut pfname = true;
        let pat_len = self.pattern.chars().count();
        for (line_no, line) in buffer.lines().enumerate() {
            let idx = line.find(self.pattern.as_str());
            if let Some(index) = idx {
                if pfname == true {
                    pprint::print_fname(dir.path().to_str().unwrap());
                    pfname = false;
                }
                let (strt, pat) = line.split_at(index);
                let (pat, end) = pat.split_at(pat_len);
                pprint::print_line(line_no+1, (strt, pat, end));
            }
        }
        Ok(())
    }
}
