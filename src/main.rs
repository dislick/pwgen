mod charset;
mod consts;

#[macro_use]
extern crate clap;

use crate::charset::*;
use crate::consts::*;
use rand::Rng;

fn main() {
    let matches = clap_app!(pwgen =>
        (version: "0.1.0")
        (author: "Patrick Muff <muff.pa@gmail.com>")
        (about: "Generates random passwords")
        (@arg LENGTH: -l --length +takes_value "Sets password length")
    )
    .get_matches();

    let password_length: u32 = matches
        .value_of("LENGTH")
        .unwrap_or("30")
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
    vec![
        Charset::new(&CHARSET_ALPHABET),
        Charset::new(&CHARSET_ALPHABET_UPPERCASE),
        Charset::new(&CHARSET_NUMBERS),
        Charset::new(&CHARSET_SPECIAL),
    ]
}

fn get_random_charset(charsets: &Vec<Charset>) -> &Charset {
    let rand_index = rand::thread_rng().gen_range(0, charsets.len());
    &charsets[rand_index]
}
