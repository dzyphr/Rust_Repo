#![allow(unused_must_use)]

//std process command gives us a direct way of interacting with subprocess without any extra crate

use std::process::Command;
fn main() 
{
    let command = "gcc";
    let argument = "--version";
    let output = Command::new(command).arg(argument).output().expect("failed to run {command$} {argument$}");
    dbg!(&format!("{:?}", output.status));
    assert!(&format!("{:?}", output.status) == "ExitStatus(unix_wait_status(0))", 
            "Bad output status! failed while running {} {}", command, argument);
    dbg!(std::str::from_utf8(&output.stdout)); //conversions from byte array to utf8
    dbg!(std::str::from_utf8(&output.stderr));
    dbg!(format!("{:?}", &output));
}


