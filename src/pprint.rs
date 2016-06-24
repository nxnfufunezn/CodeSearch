use ansi_term::Colour::{Blue, Yellow, Red, Black, Cyan};

pub fn print_fname(fname: &str) {
    println!("{}", Blue.bold().paint(fname));
}

pub fn print_line(line_no: usize, line: (&str, &str, &str)) {
    println!("{}{} {}{}{}", Yellow.bold().paint(line_no.to_string()),
    Red.bold().paint(":"), line.0, Black.on(Cyan).paint(line.1), line.2);
}
