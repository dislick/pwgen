use crate::charset::Charset;
use crate::consts::*;

pub struct Options {
  pub generator_options: GeneratorOptions,
  pub subcommand: SubCommand,
}

pub struct GeneratorOptions {
  pub length: u32,
  pub count: u32,
  pub charsets: Vec<Charset>,
}

pub enum SubCommand {
  None,
  Secret,
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

    println!(
      "{} {}",
      get_length_for_entropy(MINIMUM_ENTROPY_IN_BITS, count_chars_in_charsets(&charsets)),
      count_chars_in_charsets(&charsets)
    );

    Options {
      subcommand: match matches.subcommand_name().unwrap_or("") {
        "secret" => SubCommand::Secret,
        _ => SubCommand::None,
      },

      generator_options: GeneratorOptions {
        length: match matches.value_of("LENGTH") {
          Some(len) => len.parse().expect("Length must be a number"),
          None => {
            get_length_for_entropy(MINIMUM_ENTROPY_IN_BITS, count_chars_in_charsets(&charsets))
          }
        },
        count: matches
          .value_of("COUNT")
          .unwrap_or("1")
          .parse()
          .expect("Count must be a number"),
        charsets,
      },
    }
  }
}

pub fn count_chars_in_charsets(charsets: &Vec<Charset>) -> u32 {
  let mut count: u32 = 0;
  for charset in charsets {
    count += charset.chars.len() as u32;
  }
  count
}

pub fn get_length_for_entropy(bits: u32, distinct_chars: u32) -> u32 {
  // (bits * log(2)) / log(distinct_chars)
  (bits as f64 * 2_f64.ln() / (distinct_chars as f64).ln()).ceil() as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_256_bits_with_62_chars() {
    // To reach 256 bits of entropy with 62 different characters you need a
    // length of at least 43, as calculated below:

    // 2^256 = 62^x
    // (256 log(2))/log(62) = x
    // (256 log(2))/log(62) = ~42.995

    let length = get_length_for_entropy(256, 62);
    assert_eq!(length, 43);
  }
}
