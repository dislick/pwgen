pub struct Options {
  pub length: u32,
  pub count: u32,
  pub subcommand: SubCommand,
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
        (@subcommand secret =>
          (about: "Creates secret with 256 bits of entropy")
      )
    )
    .get_matches();

    Options {
      subcommand: match matches.subcommand_name().unwrap_or("") {
        "secret" => SubCommand::Secret,
        _ => SubCommand::None,
      },

      length: matches
        .value_of("LENGTH")
        .unwrap_or("30")
        .parse()
        .expect("Password length must be a number"),
      count: matches
        .value_of("COUNT")
        .unwrap_or("1")
        .parse()
        .expect("Count must be a number"),
    }
  }
}
