extern crate clap;

use clap::{App, AppSettings, Arg};

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test a number for primality, using Fermat's Little Theorem.")
        .author("John B. <johnboydiv@gmail.com>")
        .arg(
            Arg::with_name("PERCENT_CONFIDENCE")
                .short("c")
                .long("confidence")
                .help("Level of certainty for which to test primality.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Number to test for primality.")
                .takes_value(true)
                .required(true),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Get inputs
    let input = cli_args
        .value_of("INPUT")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    println!("INPUT = {}", input);

    let confidence = cli_args
        .value_of("PERCENT_CONFIDENCE")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    println!("CONFIDENCE = {}", confidence);
}
