use failure::Error;
use std::io::{self, Read};
use std::str::FromStr;
use structopt::StructOpt;

use smv_lib::{replace, SemVer};

#[derive(Debug)]
enum Input {
    SemVer(SemVer),
    Stdin,
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SemVer::from_str(s).map(Input::SemVer).or_else(|e| {
            if s == "-" {
                Ok(Input::Stdin)
            } else {
                Err(e)
            }
        })
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Arguments to smv")]
struct Args {
    #[structopt(
        parse(try_from_str),
        help = "x[.y.z] SemVer value to input, or - to use stdin"
    )]
    input: Input,

    #[structopt(help = "Emits string via replacement x=maj, y=min, z=patch")]
    template: String,

    #[structopt(short = "n")]
    newline: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let sem_ver = match args.input {
        Input::SemVer(sem_ver) => sem_ver,
        Input::Stdin => {
            let mut buf = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buf)?;
            SemVer::from_str(buf.trim_end())?
        }
    };

    let output = replace(&args.template, &sem_ver)?;

    if !args.newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }

    Ok(())
}
