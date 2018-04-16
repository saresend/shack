extern crate clap;

use clap::{Arg, App};

fn main() {

    let matches = App::new("Brainlag")
        .version("1.0")
        .author("Samuel Resendez saresend@usc.edu")
        .about("Brainlag is a dead easy CLI for saving values so you don't have to remember them")
        .arg(Arg::with_name("CMD").index(1).required(true))
        .arg(Arg::with_name("PARAM").index(2))
        .get_matches();

    match matches.value_of("CMD") {
        Some("ls") => {
            println!("Printing List of Stored Values");
        }
        Some("get") => {
            println!("Getting stored value");
        }

        Some("set") => {
            println!("Setting value");
        }
        _ => {
            println!("Unrecognized command, please use --help for usage.");
            return;
        }
    }




}
