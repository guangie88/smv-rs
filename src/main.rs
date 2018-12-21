use failure::Error;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "Arguments to smv")]
struct Args {
    #[structopt()]
    source: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    Ok(())
}
