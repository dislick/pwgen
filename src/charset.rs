use rand::Rng;

pub struct Charset {
  pub chars: Vec<char>,
}

impl Charset {
  pub fn new(charset: &[char]) -> Charset {
    Charset {
      chars: charset.to_vec(),
    }
  }

  pub fn get_rand_char(&self) -> char {
    let rand_index = rand::thread_rng().gen_range(0, self.chars.len());
    self.chars[rand_index]
  }
}
