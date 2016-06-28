use std::io::prelude::*;
use std::fs::{DirEntry, File};
use search::{Error, SearchAlgorithm};
use pprint;

pub struct BMH {
    pattern: String,
    bc_arr: [usize; 256],
}

impl SearchAlgorithm for BMH {
    fn new(pat: String) -> Self {
        let pattern_len = pat.chars().count();
        let mut bc_arr = [pattern_len; 256];
        for i in 0..(pattern_len-1) {
            bc_arr[pat.chars().nth(i).unwrap() as usize] = pattern_len - i - 1;
        }
        
        BMH {
            pattern: pat,
            bc_arr: bc_arr,
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
            let bline = line.as_bytes();
            let bpattern = self.pattern.as_bytes();
            let first_ch = bpattern[0];
            let middle_ch = bpattern[pat_len / 2];
            let last_ch = bpattern[pat_len - 1];
            let line_len = line.chars().count();

            let mut j = 0;
            if line_len < pat_len {
                continue;
            }
            while j <= (line_len - pat_len) {
                let c = bline[j + pat_len -1];
                if last_ch == c && middle_ch == bline[j + (pat_len/2)] &&
                    first_ch == bline[j] && &bpattern[1..(pat_len - 2)] == &bline[(j+1)..(pat_len - 2)] {
                        if pfname == true {
                            pprint::print_fname(dir.path().to_str().unwrap());
                            pfname = false;
                        }
                        let (strt, pat) = line.split_at(j);
                        let (pat, end) = pat.split_at(pat_len);
                        pprint::print_line(line_no+1, (strt, pat, end));
                    }
                j += self.bc_arr[c as usize];
            }
        }   
        Ok(())
    }
} 
