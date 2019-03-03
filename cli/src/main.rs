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
#[structopt(name = "subcommand", about = "smv subcommands")]
enum Subcommand {
    #[structopt(name = "parse")]
    Parse {
        #[structopt(
            parse(try_from_str),
            help = "x[.y.z] SemVer value to input, or - to use stdin"
        )]
        input: Input,

        #[structopt(
            help = "Emits string via replacement x=maj, y=min, z=patch"
        )]
        template: String,

        #[structopt(
            short = "n",
            help = "Insert newline char at the end if set"
        )]
        newline: bool,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "smv main arguments")]
struct Args {
    #[structopt(subcommand)]
    cmd: Subcommand,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    match args.cmd {
        Subcommand::Parse {
            input,
            template,
            newline,
        } => {
            let sem_ver = match input {
                Input::SemVer(sem_ver) => sem_ver,
                Input::Stdin => {
                    let mut buf = String::new();
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    handle.read_to_string(&mut buf)?;
                    SemVer::from_str(buf.trim_end())?
                }
            };

            let output = replace(&template, &sem_ver)?;

            if !newline {
                print!("{}", output);
            } else {
                println!("{}", output);
            }

            Ok(())
        }
    }
}
