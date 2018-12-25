extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let matches = App::new("day_1")
        .version("0.1.0")
        .author("Cat Cai <catherinetcai@gmail.com>")
        .about("Day 1 of AoC in Rust")
        .arg(Arg::with_name("path")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("path of file to read from"))
        .get_matches();

    let path = Path::new(matches.value_of("path").unwrap());
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(f) => f,
    };

    let reader = BufReader::new(f);
    let sum = reader.lines()
                    .map(|line| { line.unwrap().parse::<i32>().unwrap() }) // To int32
                    .fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
}
