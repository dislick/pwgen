use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use pwgen::*;
use std::process;

fn main() {
  // Parse cli arguments
  let options = pwgen::Options::from_args();

  match run(&options) {
    Ok(passwords) => {
      // If there is only one password being returned, copy it to clipboard
      // before printing to stdout.
      if passwords.len() == 1 {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        if let Err(e) = ctx.set_contents(passwords[0].clone()) {
          eprintln!("Could not copy password to clipboard: {}", e);
          process::exit(1);
        }
      }

      for pass in passwords {
        println!("{}", pass);
      }
    }
    Err(e) => {
      eprintln!("Application error: {}", e);
      process::exit(1);
    }
  }
}
