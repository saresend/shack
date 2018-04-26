extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod database;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Shack")
        .version("1.0")
        .author("Samuel Resendez saresend@usc.edu")
        .about("Shack is a dead easy CLI for saving values so you don't have to remember them")
        .arg(Arg::with_name("CMD").index(1).required(true))
        .arg(Arg::with_name("PARAM").index(2))
        .arg(Arg::with_name("PARAM2").index(3))
        .get_matches();

    database::initialize();

    match matches.value_of("CMD") {
        Some("ls") => {
            database::print_all_values();
        }
        Some("get") => match matches.value_of("PARAM") {
            Some(key) => {
                if let Some(val) = database::get_value(key) {
                    println!("{}", val);
                } else {
                    println!("Couldn't find value for key: {}", key);
                }
            }
            None => println!("Usage: get <PARAM>. See help for usage."),
        },
        Some("del") => match matches.value_of("PARAM") {
            Some(key) => database::delete_value(key),
            None => println!("Usage: del <PARAM>. See help for usage."),
        },

        Some("set") => {
            if let (Some(key), Some(val)) = (matches.value_of("PARAM"), matches.value_of("PARAM2"))
            {
                database::save_value(key, val);
            } else {
                println!("Usage: set <key> <val>. See help for usage.");
            }
        }
        _ => {
            println!("Unrecognized command, please use --help for usage.");
            return;
        }
    }
}
