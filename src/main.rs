mod constants;
mod generate;
mod model;
mod render;
mod utils;

use std::fs;

use generate::generate;

use crate::render::render;

fn main() {
  let space = generate();
  let svg = render(&space);
  fs::create_dir_all("dist/").unwrap();
  fs::write("dist/rkt.svg", svg).unwrap();
}
