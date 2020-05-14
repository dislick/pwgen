mod charsets;
mod consts;

use crate::charsets::*;
use crate::consts::*;
use rand::Rng;

const PASSWORD_LENGTH: i32 = 30;

fn main() {
    let mut password = String::new();
    let charsets = setup_charsets();

    for _ in 0..PASSWORD_LENGTH {
        let charset = get_random_charset(&charsets);
        password.push(charset.get_rand_char());
    }

    println!("{}", password);
}

fn setup_charsets() -> Vec<Charset> {
    let alphabet = Charset::new(&CHARSET_ALPHABET);
    let alphabet_uppercase = Charset::new(&CHARSET_ALPHABET_UPPERCASE);
    let numbers = Charset::new(&CHARSET_NUMBERS);
    let special = Charset::new(&CHARSET_SPECIAL);

    vec![alphabet, alphabet_uppercase, numbers, special]
}

fn get_random_charset(charsets: &Vec<Charset>) -> &Charset {
    let rand_index = rand::thread_rng().gen_range(0, charsets.len());
    &charsets[rand_index]
}
