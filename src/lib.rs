mod charset;
mod consts;
mod options;

#[macro_use]
extern crate clap;
extern crate clipboard;

pub use crate::charset::*;
pub use crate::consts::*;
pub use crate::options::*;

use rand::Rng;

pub fn run(options: &Options) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  match options.subcommand {
    // Mode: Default
    SubCommand::None(count) => {
      if count == 1 {
        Ok(vec![gen_password(&options.generator_options)])
      } else {
        let mut pass_list: Vec<String> = Vec::new();
        for _ in 0..count {
          pass_list.push(gen_password(&options.generator_options));
        }
        Ok(pass_list)
      }
    }

    // Mode: Secret
    SubCommand::Secret(count) => {
      let charsets = vec![
        Charset::new(&CHARSET_ALPHABET),
        Charset::new(&CHARSET_ALPHABET_UPPERCASE),
        Charset::new(&CHARSET_NUMBERS),
      ];
      let options = GeneratorOptions {
        length: get_length_for_entropy(MINIMUM_ENTROPY_IN_BITS, count_chars_in_charsets(&charsets)),
        charsets,
      };

      let mut pass_list: Vec<String> = Vec::new();
      for _ in 0..count {
        pass_list.push(gen_password(&options));
      }
      Ok(pass_list)
    }

    // Mode: WiFi
    SubCommand::WiFi(count) => {
      let charsets = vec![
        Charset::new(&CHARSET_ALPHABET),
        Charset::new(&CHARSET_NUMBERS),
      ];
      let options = GeneratorOptions {
        length: get_length_for_entropy(128, count_chars_in_charsets(&charsets)),
        charsets,
      };

      let mut pass_list: Vec<String> = Vec::new();
      for _ in 0..count {
        let pass = gen_password(&options);
        pass_list.push(format!(
          "{}-{}-{}-{}",
          &pass[0..4],
          &pass[4..8],
          &pass[8..12],
          &pass[12..16]
        ));
      }
      Ok(pass_list)
    }
  }
}

pub fn gen_password(options: &GeneratorOptions) -> String {
  let mut password = String::new();

  for _ in 0..options.length {
    let charset = get_rand_charset(&options.charsets);
    password.push(charset.get_rand_char());
  }

  password
}

fn get_rand_charset(charsets: &Vec<Charset>) -> &Charset {
  let rand_index = rand::thread_rng().gen_range(0, charsets.len());
  &charsets[rand_index]
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
