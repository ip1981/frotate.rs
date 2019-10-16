extern crate chrono;
extern crate docopt;
extern crate serde;

use std::process::exit;

use chrono::NaiveDate;
use docopt::Docopt;
use serde::Deserialize;
use std::collections::LinkedList;

mod lib;
use lib::partition_days;

const USAGE: &'static str = "
Usage: frotate [-k|-d] [-b <base>] <day>...
       frotate --help

Options:
  -k --keep         Print days to keep
  -d --delete       Print days to delete
  -b --base <base>  Base of the exponent [default: 1.1]
  -h --help         Show this help text
";

#[derive(Deserialize)]
struct Args {
    flag_keep: bool,
    flag_delete: bool,
    flag_base: f32,
    arg_day: Vec<NaiveDate>,
}

fn exponent(b: f32, n: u32) -> i64 {
    let i: i32 = n as i32 - 1;
    b.powi(i).ceil() as i64
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let parts: LinkedList<LinkedList<NaiveDate>> =
        partition_days(&|n| exponent(args.flag_base, n), &args.arg_day)
        .into_iter()
        .map(|l| l.into_iter().rev().collect())
        .collect();

    if args.flag_keep {
        for days in parts.iter() {
            match days.front() {
                Some(d) => print!("{:?} ", d),
                None => {}
            }
        }
        println!();
    } else if args.flag_delete {
        for days in parts.iter() {
            for d in days.iter().skip(1) {
                print!("{:?} ", d);
            }
        }
        println!();
    } else {
        for days in parts.iter() {
            for d in days.iter() {
                eprint!("{:?} ", d);
            }
            eprintln!();
        }
        exit(1);
    }
}
