#![allow(non_snake_case)]
use subprocess::{Popen, PopenConfig, Redirection};
fn main() 
{
    //the subprocess crate allows easy access to stdout and stderr
    let command = "gcc";
    let argument = "--version";
    let mut p = Popen::create(&[command, argument], PopenConfig {
        stdout: Redirection::Pipe, ..Default::default()
    }).expect("error running {command$} {argument$}");
    let (out, err) = p.communicate(None).expect("error geting stdout and/or stderr from running {command$} {argument$}");
    let exitStatus = p.poll();
    dbg!(&exitStatus);
    if p.poll() != None
    {
        dbg!(format!("{:?}", exitStatus));
        assert!(format!("{:?}", exitStatus) == ("Some(Exited(0))"), 
            "Subprocess execution returned unsuccessful exit status! Command being ran was:  {} {}", command, argument);
    } 
    else 
    {
        p.terminate().expect("error terminating subrocess after running {command$} {argument$}");
    }
    dbg!(&out);
    dbg!(&err);
}
