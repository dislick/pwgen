use pwgen::*;
use std::process;

fn main() {
    // Parse cli arguments
    let options = pwgen::Options::from_args();

    // Static options for subcommands
    let secret_charsets = vec![
        Charset::new(&pwgen::CHARSET_ALPHABET),
        Charset::new(&pwgen::CHARSET_ALPHABET_UPPERCASE),
        Charset::new(&pwgen::CHARSET_NUMBERS),
    ];
    let options_secret = GeneratorOptions {
        length: get_length_for_entropy(
            MINIMUM_ENTROPY_IN_BITS,
            count_chars_in_charsets(&secret_charsets),
        ),
        count: 1,
        charsets: secret_charsets,
    };

    if let Err(e) = run(&options, &options_secret) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
