use failure::{err_msg, Error};

use crate::semver;

enum State {
    Delimited,
    EscapedText(u32),
    Text,
    X,
    Y,
    Z,
}

fn is_delimiter(c: char) -> bool {
    c.is_ascii_whitespace() || c.is_ascii_punctuation()
}

pub fn replace(fmt: &str, semval: &semver::SemVer) -> Result<String, Error> {
    use self::State::*;

    // starts with Delimited
    let mut prev_state = Delimited;
    let mut output = String::with_capacity(fmt.len() * 2);

    for c in fmt.chars() {
        match prev_state {
            Delimited => match c {
                'x' => prev_state = X,
                'y' => prev_state = Y,
                'z' => prev_state = Z,
                c => {
                    if is_delimiter(c) {
                        // no change to state
                        output.push(c)
                    }
                }
            },
            EscapedText(level) => {}
            Text => {
                output.push(c);
            }
            X => {
                if is_delimiter(c) {
                    output.push_str(&format!("{}", semval.major));
                    prev_state = Delimited;
                } else {
                    output.push('x');
                    prev_state = Text
                }
            }
            Y => {
                if is_delimiter(c) {
                    output.push_str(&format!(
                        "{}",
                        semval.minor.ok_or_else(|| err_msg(
                            "No minor version, cannot replace y"
                        ))?
                    ));
                    prev_state = Delimited;
                } else {
                    output.push('x');
                    prev_state = Text
                }
            }
            Z => {}
        }
        if c.is_ascii_punctuation() {

        } else {

        }
    }

    Ok("".to_string())
}
