use failure::{err_msg, Error};

#[derive(Debug)]
pub struct SemVer {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
}

impl std::str::FromStr for SemVer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split('.');

        let mut vers = vals.map(|val| val.parse::<u32>());

        let major = vers
            .next()
            .ok_or_else(|| err_msg("Major version not present"))??;

        let minor = vers.next().map(|r| r.unwrap());
        let patch = vers.next().map(|r| r.unwrap());

        Ok(SemVer {
            major,
            minor,
            patch,
        })
    }
}
