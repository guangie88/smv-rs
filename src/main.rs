use failure::Error;
use std::io::{self, Read};
use std::str::FromStr;
use structopt::StructOpt;

mod semver;

#[derive(Debug)]
enum Input {
    SemVer(semver::SemVer),
    Stdin,
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        semver::SemVer::from_str(s).map(Input::SemVer).or_else(|e| {
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

    #[structopt(
        help = "Emits string via replacement {x}=maj, {y}=min, {z}=patch"
    )]
    emit: String,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let semval = match args.input {
        Input::SemVer(semval) => semval,
        Input::Stdin => {
            let mut buf = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buf)?;
            semver::SemVer::from_str(&buf)?
        }
    };

    let x_rep = args.emit.replace("{x}", &format!("{}", semval.major));

    let xy_rep = match semval.minor {
        Some(minor) => x_rep.replace("{y}", &format!("{}", minor)),
        None => x_rep,
    };

    let xyz_rep = match semval.patch {
        Some(patch) => xy_rep.replace("{z}", &format!("{}", patch)),
        None => xy_rep,
    };

    print!("{}", xyz_rep);
    Ok(())
}
