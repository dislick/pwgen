use rand::Rng;

const PASSWORD_LENGTH: i32 = 30;
const CHARSET_ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let mut password = String::new();

    for _ in 0..PASSWORD_LENGTH {
        let rand_char = get_random_char_from_charset(&CHARSET_ALPHABET);
        password.push(rand_char);
    }

    println!("{}", password);
}

fn get_random_char_from_charset(charset: &[char]) -> char {
    let rand_index = rand::thread_rng().gen_range(0, charset.len());
    charset[rand_index]
}
