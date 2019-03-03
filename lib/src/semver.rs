use failure::{err_msg, Error};

#[derive(Debug)]
pub struct SemVer {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
}

impl SemVer {
    pub fn from_major(x: u32) -> SemVer {
        SemVer {
            major: x,
            minor: None,
            patch: None,
        }
    }

    pub fn from_major_minor(x: u32, y: u32) -> SemVer {
        SemVer {
            major: x,
            minor: Some(y),
            patch: None,
        }
    }

    pub fn from_major_minor_patch(x: u32, y: u32, z: u32) -> SemVer {
        SemVer {
            major: x,
            minor: Some(y),
            patch: Some(z),
        }
    }
}

impl std::str::FromStr for SemVer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split('.').take(3);

        let mut vers = vals.map(|val| val.parse::<u32>());

        let major = vers
            .next()
            .ok_or_else(|| err_msg("Major version not present"))??;

        let minor = match vers.next() {
            Some(y) => Some(y?),
            None => None,
        };

        let patch = match vers.next() {
            Some(z) => Some(z?),
            None => None,
        };

        Ok(SemVer {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn sem_ver_from_str_major() {
        let sm = SemVer::from_str("3").unwrap();
        assert_eq!(sm.major, 3);
        assert_eq!(sm.minor, None);
        assert_eq!(sm.patch, None);
    }

    #[test]
    fn sem_ver_from_str_major_minor() {
        let sm = SemVer::from_str("3.1").unwrap();
        assert_eq!(sm.major, 3);
        assert_eq!(sm.minor, Some(1));
        assert_eq!(sm.patch, None);
    }

    #[test]
    fn sem_ver_from_str_major_minor_patch() {
        let sm = SemVer::from_str("3.1.4").unwrap();
        assert_eq!(sm.major, 3);
        assert_eq!(sm.minor, Some(1));
        assert_eq!(sm.patch, Some(4));
    }

    #[test]
    fn sem_ver_from_str_large_major_minor_patch() {
        let sm = SemVer::from_str("123.456.789").unwrap();
        assert_eq!(sm.major, 123);
        assert_eq!(sm.minor, Some(456));
        assert_eq!(sm.patch, Some(789));
    }
}
