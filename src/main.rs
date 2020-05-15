use std::process;

fn main() {
    let options = pwgen::Options::setup();
    let options_secret = pwgen::Options {
        length: 256, // TODO: this is not 256 bits of entropy, but much more.
        count: 1,
        subcommand: pwgen::SubCommand::None,
    };

    if let Err(e) = pwgen::run(&options, &options_secret) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
