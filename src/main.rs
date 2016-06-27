extern crate ansi_term;
extern crate getopts;
extern crate crossbeam;

use std::path::Path;
use getopts::Options;
use std::env;

mod search;
mod kmp;
mod bmh;
mod index;
mod pprint;
mod results;
use kmp::KMP;
use bmh::BMH;
use search::SearchAlgorithm;

fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} FILE [search_pattern]", program);
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let path = Path::new(".");
    KMP::new(input).recursive_search(path);
}
