use ansi_term::Colour::{Blue, Yellow, Red};

pub fn print_fname(fname: &str) {
    println!("{}", Blue.bold().paint(fname));
}

pub fn print_line(line_no: usize, line: &str) {
    println!("{}{} {}", Yellow.bold().paint(line_no.to_string()),
    Red.bold().paint(":"), line);
}
