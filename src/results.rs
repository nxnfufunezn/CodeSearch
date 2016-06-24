use std::collections::HashMap;
use std::collections::hash_map;

#[allow(dead_code)]
pub struct InFileSearch {
    filename: String,
    line: HashMap<usize, String>,
}

#[allow(dead_code)]
pub struct SearchResult {
    results: Vec<InFileSearch>,
}

#[allow(dead_code)]
impl InFileSearch {
    pub fn new() -> InFileSearch {
        InFileSearch {
            filename: String::new(),
            line: HashMap::new(),
        }
    }

    pub fn change_fname(&mut self, filename: String) {
        self.filename = filename;
    }
    
    pub fn insert_result(&mut self, line_no: usize, line: String) {
        self.line.insert(line_no, line);
    }
}

impl IntoIterator for InFileSearch {
    type Item = (usize, String);
    type IntoIter = hash_map::IntoIter<usize, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.line.into_iter()
    }
}

impl SearchResult {
    // TODO
}
