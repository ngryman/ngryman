mod constants;
mod generate;
mod model;
mod render;
mod utils;

use std::fs;

use generate::{generate, Config};

use crate::render::render;

fn main() {
  let config = Config { stars_count: 100 };
  let space = generate(config);
  let svg = render(space);
  fs::write("banner.svg", svg).unwrap();
}
