extern crate clap;
use clap::{App, Arg, ArgMatches};
use String;
use std::process::Command;


fn main() {
    let matches = App::new("My super pipe of doom")
        .version("1.0.0")
        .author("Rémi Bruyère")
        .about("Program to simulate a pipe")
        .arg(Arg::with_name("input")
            .short("in")
            .long("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("out")
            .long("output")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_value: String = get_input_value(&matches);
    let output_value: String = get_output_value(&matches);

    let result_fortune =
        Command::new(input_value)
            .output()
            .expect("failed to execute process");

    let message_fortune = String::from_utf8_lossy(&result_fortune.stdout).to_string();

    let cowsay_result =
        Command::new(output_value)
            .arg(message_fortune)
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&cowsay_result.stdout));
}

fn get_input_value(matches: &ArgMatches) -> String {
    return matches.value_of("input").unwrap().to_string();
}

fn get_output_value(matches: &ArgMatches) -> String {
    return matches.value_of("output").unwrap().to_string();
}
