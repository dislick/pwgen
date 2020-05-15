use std::process;

fn main() {
    // Parse cli arguments
    let options = pwgen::Options::from_args();

    // Static options for subcommands
    let options_secret = pwgen::GeneratorOptions {
        length: 256, // TODO: this is not 256 bits of entropy, but much more.
        count: 1,
        charsets: vec![pwgen::Charset::new(&pwgen::CHARSET_ALPHABET)],
    };

    if let Err(e) = pwgen::run(&options, &options_secret) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
