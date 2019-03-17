use crate::Version;
use failure::{err_msg, Error};

pub const ESCAPE: char = '\\';

enum State {
    Delimited,
    Text,
    Escaped,
    X,
    Y,
    Z,
}

fn is_delimiter(c: char) -> bool {
    c.is_ascii_whitespace() || c.is_ascii_punctuation()
}

fn push_x(semval: &Version, output: &mut String) {
    output.push_str(&format!("{}", semval.major));
}

fn handle_x(c: char, semval: &Version, output: &mut String) -> State {
    use self::State::*;

    match c {
        ESCAPE => {
            output.push('x');
            Escaped
        }
        c if is_delimiter(c) => {
            push_x(semval, output);
            output.push(c);
            Delimited
        }
        c => {
            output.push('x');
            output.push(c);
            Text
        }
    }
}

fn push_y(semval: &Version, output: &mut String) {
    output.push_str(&format!("{}", semval.minor));
}

fn handle_y(
    c: char,
    semval: &Version,
    output: &mut String,
) -> Result<State, Error> {
    use self::State::*;

    Ok(match c {
        ESCAPE => {
            output.push('y');
            Escaped
        }
        c if is_delimiter(c) => {
            push_y(semval, output);
            output.push(c);
            Delimited
        }
        c => {
            output.push('y');
            output.push(c);
            Text
        }
    })
}

fn push_z(semval: &Version, output: &mut String) {
    output.push_str(&format!("{}", semval.patch));
}

fn handle_z(
    c: char,
    semval: &Version,
    output: &mut String,
) -> Result<State, Error> {
    use self::State::*;

    Ok(match c {
        ESCAPE => {
            output.push('z');
            Escaped
        }
        c if is_delimiter(c) => {
            push_z(semval, output);
            output.push(c);
            Delimited
        }
        c => {
            output.push('z');
            output.push(c);
            Text
        }
    })
}

pub fn replace(fmt: &str, semval: &Version) -> Result<String, Error> {
    use self::State::*;

    // starts with Delimited
    let mut state = Delimited;
    let mut output = String::with_capacity(fmt.len() * 2);

    for c in fmt.chars() {
        match state {
            Delimited => match c {
                ESCAPE => state = Escaped,
                'x' => state = X,
                'y' => state = Y,
                'z' => state = Z,
                c => output.push(c),
            },

            Text => match c {
                ESCAPE => state = Escaped,
                c => {
                    output.push(c);
                    state = if is_delimiter(c) { Delimited } else { Text };
                }
            },

            Escaped => match c {
                c @ 'x' | c @ 'y' | c @ 'z' | c @ ESCAPE => {
                    output.push(c);
                    state = Text;
                }
                c => Err(err_msg(format!(
                    "Invalid char '{}' found after escape char",
                    c
                )))?,
            },

            X => state = handle_x(c, semval, &mut output),
            Y => state = handle_y(c, semval, &mut output)?,
            Z => state = handle_z(c, semval, &mut output)?,
        }
    }

    // Check special ending state
    match state {
        Escaped => Err(err_msg("Invalid escape char found at last index"))?,
        X => push_x(semval, &mut output),
        Y => push_y(semval, &mut output),
        Z => push_z(semval, &mut output),
        _ => {}
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Version;

    #[test]
    fn replace_xyz() {
        assert_eq!(
            replace("x.y.z", &Version::parse("3.1.4").unwrap()).unwrap(),
            "3.1.4"
        );
    }

    #[test]
    fn replace_xy() {
        assert_eq!(
            replace("x.y", &Version::parse("3.1.4").unwrap()).unwrap(),
            "3.1"
        );
    }

    #[test]
    fn replace_x() {
        assert_eq!(
            replace("x", &Version::parse("3.1.4").unwrap()).unwrap(),
            "3"
        );
    }

    #[test]
    fn replace_large_xyz() {
        assert_eq!(
            replace("x.y.z", &Version::parse("100.213.579").unwrap()).unwrap(),
            "100.213.579"
        );
    }

    #[test]
    fn replace_xyz_with_text() {
        assert_eq!(
            replace("xx:x,yy:y zz:z", &Version::parse("3.1.4").unwrap())
                .unwrap(),
            "xx:3,yy:1 zz:4"
        );
    }

    #[test]
    fn replace_xyz_with_complex_text() {
        assert_eq!(
            replace(
                "xabcx x-x defyy y,y zzghi z:z helloworld",
                &Version::parse("3.1.4").unwrap()
            )
            .unwrap(),
            "xabcx 3-3 defyy 1,1 zzghi 4:4 helloworld"
        );
    }

    #[test]
    fn replace_xyz_with_escaped_text() {
        assert_eq!(
            replace(
                "\\x:x,\\y:y x\\\\ \\\\z \\z:z",
                &Version::parse("3.1.4").unwrap()
            )
            .unwrap(),
            "x:3,y:1 x\\ \\z z:4"
        );
    }
}
