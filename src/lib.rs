mod charset;
mod consts;
mod options;

#[macro_use]
extern crate clap;
extern crate clipboard;

pub use crate::charset::*;
pub use crate::consts::*;
pub use crate::options::*;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use rand::Rng;

pub fn run(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
  match options.subcommand {
    // Mode: Default
    SubCommand::None => {
      if options.generator_options.count == 1 {
        let pass = gen_password(&options.generator_options);
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        println!("{}", pass);
        ctx.set_contents(pass)?;
      } else {
        for _ in 0..options.generator_options.count {
          println!("{}", gen_password(&options.generator_options));
        }
      }
    }
    // Mode: Secret
    SubCommand::Secret => {
      let secret_charsets = vec![
        Charset::new(&CHARSET_ALPHABET),
        Charset::new(&CHARSET_ALPHABET_UPPERCASE),
        Charset::new(&CHARSET_NUMBERS),
      ];
      let options_secret = GeneratorOptions {
        length: get_length_for_entropy(
          MINIMUM_ENTROPY_IN_BITS,
          count_chars_in_charsets(&secret_charsets),
        ),
        count: 1,
        charsets: secret_charsets,
      };
      println!("{}", gen_password(&options_secret))
    }
  }

  Ok(())
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
