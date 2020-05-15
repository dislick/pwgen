use std::process;

fn main() {
    // Parse cli arguments
    let options = pwgen::Options::from_args();

    // Static options for subcommands
    let options_secret = pwgen::GeneratorOptions {
        // The listed charsets below contain 62 different characters. To
        // determine which password length is needed to get to 256 bits of
        // entropy, one needs to solve the following equation:

        // 2^256 = 62^x
        // (256 log(2))/log(62) = x
        // (256 log(2))/log(62) = ~42.995

        // Therefore pwgen generates a password of length 43, which is a _bit_
        // more entropy than 2^256.
        length: 43,
        count: 1,
        charsets: vec![
            pwgen::Charset::new(&pwgen::CHARSET_ALPHABET),
            pwgen::Charset::new(&pwgen::CHARSET_ALPHABET_UPPERCASE),
            pwgen::Charset::new(&pwgen::CHARSET_NUMBERS),
        ],
    };

    if let Err(e) = pwgen::run(&options, &options_secret) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
