use crate::constants::*;
use crate::model::*;
use crate::utils::random_range;

const STAR_ALPHABET: [&str; 9] = ["★", "☆", "☾", "○", "•", "¸", ".", "°", "•"];

pub struct Config {
  pub stars_count: u16,
}

pub fn generate(config: Config) -> Space {
  let halve1 = generate_half(&config);
  let halve2 = generate_half(&config);
  Space { halves: [halve1, halve2] }
}

fn generate_half(config: &Config) -> Half {
  let stars = vec![0; (config.stars_count / 2) as usize].iter().map(|_| generate_star()).collect();
  Half { stars }
}

fn generate_star() -> Star {
  let x = random_range(0..CANVAS_WIDTH);
  let y = random_range(0..CANVAS_HEIGHT);
  let symbol = STAR_ALPHABET[random_range(0..STAR_ALPHABET.len() as u16) as usize];
  Star { x, y, symbol }
}
