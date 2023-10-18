use std::{io, process::Command, str::from_utf8};

fn spawn() -> Result<(String, String), io::Error> {
    let output = Command::new("bash")
        .args(vec!["-i", "-l", "-c", "printenv"])
        .output()?;
    let stdout = from_utf8(&output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let stderr = from_utf8(&output.stderr)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok((stdout.to_owned(), stderr.to_owned()))
}

pub fn runner() {
    match spawn() {
        Ok((_stdout, stderr)) => {
            // println!("stdout: {stdout}");
            println!("stderr: {stderr}");
        }
        Err(e) => {
            eprintln!("Failed with: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> () {
        let mut i = 0;
        while i <= 10 {
            println!("Run #{i}");
            runner();
            i += 1;
        }
    }
}
