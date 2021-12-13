use anyhow::{Result};

use clap_derive::Parser;
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;


use rpncalc::rpn_calculator::RpnCalculator;


#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.1",
    author = "Tak Enomoto",
    about = "Super awesome sample RPN calculator.\nFor stop exec in the stdin mode, push ctrl+d.",
)]
struct Opts {
    #[clap(
        short, long,
        help = "Sets the level of verbosity",
    )]
    verbose: bool,

    #[clap(
        name = "FILE",
        help = "Formulas written in RPN",
    )]
    formula_file: Option<PathBuf>,
}


fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        return run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        return run(reader, opts.verbose);
    }
}


fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }

    return Ok(());
}