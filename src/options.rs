use crate::charset::Charset;
use crate::consts::*;
use crate::{count_chars_in_charsets, get_length_for_entropy};

pub struct Options {
  pub generator_options: GeneratorOptions,
  pub subcommand: SubCommand,
}

pub struct GeneratorOptions {
  pub length: u32,
  pub charsets: Vec<Charset>,
}

pub enum SubCommand {
  None(u32),
  Secret(u32),
  WiFi(u32),
}

impl Options {
  pub fn from_args() -> Options {
    let matches = clap_app!(pwgen =>
        (version: "0.1.0")
        (author: "Patrick Muff <muff.pa@gmail.com>")
        (about: "Generates random passwords")
        (@arg LENGTH: -l --length +takes_value "Length of passwords")
        (@arg COUNT: -c --count +takes_value "Amount of passwords")
        (@arg ALPHABET: -a --alphabet "Use ALPHABET (a-z) charset")
        (@arg ALPHABET_UPPERCASE: -A --("alphabet-uppercase") "Use ALPHABET_UPPERCASE (A-Z) charset")
        (@arg NUMBERS: -n --numbers "Use NUMBERS (0-9) charset")
        (@arg SPECIAL: -s --special "Use SPECIAL (*, %, -, ...) charset")
        (@subcommand secret =>
          (about: "Creates secret with at least 256 bits of entropy")
          (@arg COUNT: -c --count +takes_value "Amount of secrets")
        )
        (@subcommand wifi =>
          (about: "Creates a wifi friendly password")
          (@arg COUNT: -c --count +takes_value "Amount of wifi friendly passwords")
        )
    )
    .get_matches();

    let mut charsets: Vec<Charset> = Vec::new();

    if matches.is_present("ALPHABET") {
      charsets.push(Charset::new(&CHARSET_ALPHABET));
    }
    if matches.is_present("ALPHABET_UPPERCASE") {
      charsets.push(Charset::new(&CHARSET_ALPHABET_UPPERCASE));
    }
    if matches.is_present("NUMBERS") {
      charsets.push(Charset::new(&CHARSET_NUMBERS));
    }
    if matches.is_present("SPECIAL") {
      charsets.push(Charset::new(&CHARSET_SPECIAL));
    }

    // If the user passes no charset arguments, use default ones.
    if charsets.len() == 0 {
      charsets.push(Charset::new(&CHARSET_ALPHABET));
      charsets.push(Charset::new(&CHARSET_ALPHABET_UPPERCASE));
      charsets.push(Charset::new(&CHARSET_NUMBERS));
      charsets.push(Charset::new(&CHARSET_SPECIAL));
    }

    Options {
      subcommand: match matches.subcommand_name().unwrap_or("") {
        "secret" => {
          let count = match matches.subcommand_matches("secret") {
            Some(matches) => get_count_from_matches(&matches),
            None => 1,
          };
          SubCommand::Secret(count)
        }
        "wifi" => {
          let count = match matches.subcommand_matches("wifi") {
            Some(matches) => get_count_from_matches(&matches),
            None => 1,
          };
          SubCommand::WiFi(count)
        }
        _ => {
          let count = get_count_from_matches(&matches);
          SubCommand::None(count)
        }
      },

      generator_options: GeneratorOptions {
        length: match matches.value_of("LENGTH") {
          Some(len) => len.parse().expect("Length must be a number"),
          None => {
            get_length_for_entropy(MINIMUM_ENTROPY_IN_BITS, count_chars_in_charsets(&charsets))
          }
        },
        charsets,
      },
    }
  }
}

fn get_count_from_matches(matches: &clap::ArgMatches) -> u32 {
  matches
    .value_of("COUNT")
    .unwrap_or("1")
    .parse()
    .expect("Count must be a number")
}
