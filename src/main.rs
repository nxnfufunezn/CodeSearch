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
mod frmlib;
use kmp::KMP;
use bmh::BMH;
use frmlib::LIB;
use search::SearchAlgorithm;

fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} [search_pattern]", program);
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("k", "knuth-morris-pratt", "Search using Knuth Morris Pratt");
    opts.optflag("r", "raita", "Search using Raita");
    opts.optflag("l", "std-lib", "Search using standard library implementation find() fn");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let path = Path::new(".");

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("k") {
         KMP::new(input).recursive_search(path);
    } else if matches.opt_present("r") {
         BMH::new(input).recursive_search(path);
    } else if matches.opt_present("l") {
         LIB::new(input).recursive_search(path);
    } else {
         LIB::new(input).recursive_search(path);
    }
}
