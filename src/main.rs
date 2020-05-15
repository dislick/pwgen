mod charset;
mod consts;
mod options;

#[macro_use]
extern crate clap;

use crate::charset::*;
use crate::consts::*;
use crate::options::*;
use rand::Rng;

fn main() {
    let options = Options::setup();
    let options_secret = Options {
        length: 256, // TODO: this is not 256 bits of entropy, but much more.
        count: 1,
        subcommand: SubCommand::None,
    };

    match options.subcommand {
        SubCommand::None => {
            for _ in 0..options.count {
                println!("{}", gen_password(&options));
            }
        }
        SubCommand::Secret => println!("{}", gen_password(&options_secret)),
    }
}

fn gen_password(options: &Options) -> String {
    let mut password = String::new();
    let charsets = setup_charsets();

    for _ in 0..options.length {
        let charset = get_random_charset(&charsets);
        password.push(charset.get_rand_char());
    }

    password
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
