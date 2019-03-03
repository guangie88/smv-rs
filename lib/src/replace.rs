use failure::{err_msg, Error};

use crate::semver;

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

fn push_x(semval: &semver::SemVer, output: &mut String) {
    output.push_str(&format!("{}", semval.major));
}

fn handle_x(c: char, semval: &semver::SemVer, output: &mut String) -> State {
    use self::State::*;

    if is_delimiter(c) {
        push_x(semval, output);
        output.push(c);
        Delimited
    } else {
        output.push('x');
        Text
    }
}

fn push_y(semval: &semver::SemVer, output: &mut String) -> Result<(), Error> {
    output.push_str(&format!(
        "{}",
        semval
            .minor
            .ok_or_else(|| err_msg("No minor version, cannot replace y"))?
    ));
    Ok(())
}

fn handle_y(
    c: char,
    semval: &semver::SemVer,
    output: &mut String,
) -> Result<State, Error> {
    use self::State::*;

    if is_delimiter(c) {
        push_y(semval, output)?;
        output.push(c);
        Ok(Delimited)
    } else {
        output.push('y');
        Ok(Text)
    }
}

fn push_z(semval: &semver::SemVer, output: &mut String) -> Result<(), Error> {
    output.push_str(&format!(
        "{}",
        semval
            .patch
            .ok_or_else(|| err_msg("No patch version, cannot replace z"))?
    ));
    Ok(())
}

fn handle_z(
    c: char,
    semval: &semver::SemVer,
    output: &mut String,
) -> Result<State, Error> {
    use self::State::*;

    if is_delimiter(c) {
        push_z(semval, output)?;
        output.push(c);
        Ok(Delimited)
    } else {
        output.push('z');
        Ok(Text)
    }
}

pub fn replace(fmt: &str, semval: &semver::SemVer) -> Result<String, Error> {
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
                c => {
                    Err(err_msg(format!("Invalid char after escape '{}'", c)))?
                }
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
        Y => {
            push_y(semval, &mut output)?;
        }
        Z => {
            push_z(semval, &mut output)?;
        }
        _ => {}
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::semver::SemVer;

    #[test]
    fn replace_xyz() {
        assert_eq!(
            replace("x.y.z", &SemVer::from_major_minor_patch(3, 1, 4)).unwrap(),
            "3.1.4"
        );
    }
}
