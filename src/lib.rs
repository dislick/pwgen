mod charset;
mod consts;
mod options;

#[macro_use]
extern crate clap;

pub use crate::charset::*;
pub use crate::consts::*;
pub use crate::options::*;
use rand::Rng;

pub fn run(
  options: &Options,
  options_secret: &GeneratorOptions,
) -> Result<(), Box<dyn std::error::Error>> {
  match options.subcommand {
    SubCommand::None => {
      for _ in 0..options.generator_options.count {
        println!("{}", gen_password(&options.generator_options));
      }
    }
    SubCommand::Secret => println!("{}", gen_password(&options_secret)),
  }

  Ok(())
}

pub fn gen_password(options: &GeneratorOptions) -> String {
  let mut password = String::new();

  for _ in 0..options.length {
    let charset = get_random_charset(&options.charsets);
    password.push(charset.get_rand_char());
  }

  password
}

fn get_random_charset(charsets: &Vec<Charset>) -> &Charset {
  let rand_index = rand::thread_rng().gen_range(0, charsets.len());
  &charsets[rand_index]
}
