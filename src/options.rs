pub struct Options {
  pub length: u32,
  pub count: u32,
}

impl Options {
  pub fn setup() -> Options {
    let matches = clap_app!(pwgen =>
        (version: "0.1.0")
        (author: "Patrick Muff <muff.pa@gmail.com>")
        (about: "Generates random passwords")
        (@arg LENGTH: -l --length +takes_value "Length of passwords")
        (@arg COUNT: -c --count +takes_value "Amount of passwords")
    )
    .get_matches();

    Options {
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
