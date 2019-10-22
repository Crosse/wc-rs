use std::env;
use std::fs;
use std::io::prelude::*;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print the help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input = if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    } else {
        matches.free[0].clone()
    };

    let mut file = fs::File::open(&input).expect(&format!("could not open {}", &input));

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let mut was_carriage_return = false;
    let mut was_space = true;

    let mut buf = [0; 1048576];

    loop {
        let r = file.read(&mut buf[..]).expect("error reading file");

        if r == 0 {
            break;
        }

        for b in &buf[0..r] {
            chars += 1;
            if (*b as char).is_whitespace() {
                was_space = true;
                if *b == b'\n' {
                    if !was_carriage_return {
                        was_carriage_return = false;
                        lines += 1;
                    }
                }
                if *b == b'\r' {
                    was_carriage_return = true;
                    lines += 1;
                }
            } else {
                if was_space {
                    words += 1;
                }
                was_carriage_return = false;
                was_space = false;
            }
        }
    }

    println!("{:8} {:7} {:7} {}", lines, words, chars, input);
}
