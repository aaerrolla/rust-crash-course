use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub fn main() {
    let mut left_child = Command::new("/bin/ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let mut right_child = Command::new("/bin/cat")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    // extra scope to ensure that left_in and right_out are closed after
    // copying all available data
    {
        let left_in = BufReader::new(left_child.stdout.take().unwrap());
        let mut right_out = right_child.stdin.take().unwrap();
        for line in left_in.lines() {
            writeln!(&mut right_out, "{}", line.unwrap()).unwrap();
        }
    }
    let left_ecode = left_child.wait().expect("failed to wait on child");
    let right_ecode = right_child.wait().expect("failed to wait on child");

    assert!(left_ecode.success());
    assert!(right_ecode.success());
}