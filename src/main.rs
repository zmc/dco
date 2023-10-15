use std::error::Error;
use std::io::{self, StdinLock};
use std::process::{Command, Output};
use std::string::String;

fn main() {
    let signoff = "Signed-off-by:";
    let name = cmd("git", vec!["config", "user.name"]);
    let email = cmd("git", vec!["config", "user.email"]);
    let mut want_dco = true;
    if name.is_err() {
        eprintln!(
            "Error getting name: {:?}",
            name.as_ref().err().unwrap().to_string().trim()
        );
        want_dco = false;
    }
    if email.is_err() {
        eprintln!(
            "Error getting email: {:?}",
            email.as_ref().err().unwrap().to_string().trim()
        );
        want_dco = false;
    }
    if !want_dco {
        eprintln!("Encountered errors; outputting message unmodified.");
    } else {
        let dco = format!(
            "{} {} <{}>",
            signoff,
            name.as_ref().unwrap().trim(),
            email.as_ref().unwrap().trim()
        );
        let lines = io::stdin().lines();
        print_commit(signoff, dco, lines);
    }
}

fn print_commit(signoff: &str, dco: String, lines: std::io::Lines<StdinLock>) {
    let mut want_dco = true;
    let mut prepend_newline = false;
    for line_ in lines {
        if line_.is_err() {
            eprintln!("err: {:?}", line_.err().unwrap().to_string().trim());
            continue;
        }
        let line = line_.unwrap();
        if line.contains(signoff) {
            want_dco = false;
        }
        let char = line.chars().nth(0);
        match char {
            None => {
                prepend_newline = false;
                println!("")
            }
            Some(inner) => {
                if want_dco && inner == '#' {
                    match prepend_newline {
                        true => println!("\n{}", dco),
                        false => println!("{}", dco),
                    }
                    want_dco = false;
                } else {
                    prepend_newline = true;
                }
                println!("{}", line);
            }
        }
    }
}

fn cmd(cmd: &str, args: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let output: Output = Command::new(cmd).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!(
            "Command failed with exit code {}: {}",
            output.status,
            String::from_utf8(output.stderr)?
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_echo() {
        let out = cmd("echo", vec!["hello"]);
        assert!(out.is_ok());
        assert_eq!(out.unwrap(), "hello\n");
    }

    #[test]
    fn cmd_false() {
        let out = cmd("false", vec![]);
        assert!(out.is_err());
    }

    // TODO: use assert_cmd?
}
