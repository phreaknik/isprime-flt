extern crate clap;
extern crate rand;
extern crate mod_exp;

use clap::{App, AppSettings, Arg};
use mod_exp::mod_exp;

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

    let confidence = cli_args
        .value_of("PERCENT_CONFIDENCE")
        .unwrap_or("0.1")
        .parse()
        .unwrap_or(0.0);

    // Loop until desired confidence is acheived
    let mut pass_count = 0;
    let mut c = 0.0;
    while c < confidence {
        // Pick a random number between 2 & 'input', and test input for primality
        let a = (rand::random::<u32>() % (input - 2)) + 2;
        if mod_exp(a, input, input) != a {
            // Input is NOT prime!
            break;
        }

        // Update pass count
        pass_count += 1;

        // Calculate current degree of confidence
        let pow2: f32 = (pass_count as f32).exp2();
        c = (pow2 - 1.0) / pow2;
    }

    // Check if desired confidence level was acheived
    if c < confidence {
        println!("{} is NOT prime.", input);
    }
    else {
        println!("{} is prime with {}% confidence.", input, 100.0*c);
    }
}
