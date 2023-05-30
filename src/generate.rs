use crate::constants::*;
use crate::model::*;
use crate::utils::random_range;

const STAR_ALPHABET: [&str; 9] = ["★", "☆", "☾", "○", "•", "¸", ".", "°", "•"];

pub fn generate() -> Space {
  let halve1 = generate_half();
  let halve2 = generate_half();
  Space { halves: [halve1, halve2] }
}

fn generate_half() -> Half {
  let stars = [0; 200].iter().map(|_| generate_star()).collect();
  Half { stars }
}

fn generate_star() -> Star {
  let x = random_range(0..CANVAS_WIDTH);
  let y = random_range(0..CANVAS_HEIGHT);
  let symbol = STAR_ALPHABET[random_range(0..STAR_ALPHABET.len() as u16) as usize];
  Star { x, y, symbol }
}
