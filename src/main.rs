use failure::Error;
use structopt::StructOpt;

mod semver;

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "Arguments to smv")]
struct Args {
    #[structopt(parse(try_from_str))]
    semval: semver::SemVer,

    #[structopt()]
    emit: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    print!("{}", args.emit
        .replace('x', &format!("{}", args.semval.major))
        .replace('y', &format!("{}", args.semval.minor.unwrap()))
        .replace('z', &format!("{}", args.semval.patch.unwrap())));
    Ok(())
}
