mod charset;
mod consts;

extern crate clap;

use crate::charset::*;
use crate::consts::*;
use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("pwgen")
        .version("0.1.0")
        .author("Patrick Muff <muff.pa@gmail.com>")
        .about("Generates random passwords")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .default_value("30")
                .help("Determines password length")
                .takes_value(true),
        )
        .get_matches();

    let password_length: u32 = matches
        .value_of("length")
        .unwrap()
        .parse()
        .expect("Password length must be a number");

    let mut password = String::new();
    let charsets = setup_charsets();

    for _ in 0..password_length {
        let charset = get_random_charset(&charsets);
        password.push(charset.get_rand_char());
    }

    println!("{}", password);
}

fn setup_charsets() -> Vec<Charset> {
    let alphabet = Charset::new(&CHARSET_ALPHABET);
    let alphabet_uppercase = Charset::new(&CHARSET_ALPHABET_UPPERCASE);
    let numbers = Charset::new(&CHARSET_NUMBERS);
    let special = Charset::new(&CHARSET_SPECIAL);

    vec![alphabet, alphabet_uppercase, numbers, special]
}

fn get_random_charset(charsets: &Vec<Charset>) -> &Charset {
    let rand_index = rand::thread_rng().gen_range(0, charsets.len());
    &charsets[rand_index]
}
