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
    pattern: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    println!("{:?}", args);
    Ok(())
}
