// this is get args with std library
// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
// }

// this is typical builder pattern
// use clap::{arg, App};
//
// fn main() {
//     let matches = App::new("My RPN program")
//         .version("1.0.0")
//         .author("Suyeong RHIE")
//         .about("The sample RPN calculator")
//         .arg(arg!([FILE] "Formulas written in RPN").required(false))
//         .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
//         .get_matches();
//
//     match matches.value_of("FILE") {
//         Some(file) => println!("File specified: {}", file),
//         None => println!("No file specified.")
//     }
//     let verbose = matches.is_present("verbose");
//     println!("Is verbosity specified?: {}", verbose);
// }

//this is derive macro
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/// for use derive, you should insert 'features = ["derive"]' in Cargo.toml.
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Suyeong RHIE",
    about = "The sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Number
    // #[clap(name = "NUMBER")]
    // num: i32,

    /// Formulas written in RPN
    // #[clap(name = "FILE", default_value = "default.txt")]
    #[clap(name = "FILE")]
    formula_file: Option<String>
}

fn main() {
    let opts = Opts::parse();

    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified.")
    // }
    // println!("Is verbosity specified?: {}", opts.verbose);

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
        println!("No file is specified.");
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid systax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
}