pub const MINIMUM_ENTROPY_IN_BITS: u32 = 256;

pub const CHARSET_ALPHABET: [char; 26] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const CHARSET_ALPHABET_UPPERCASE: [char; 26] = [
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
  'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const CHARSET_NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub const CHARSET_SPECIAL: [char; 29] = [
  '!', '\\', '#', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?',
  '@', '[', ']', '^', '_', '`', '{', '|', '}', '~',
];
