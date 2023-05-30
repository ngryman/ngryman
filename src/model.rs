#[derive(Debug)]
pub struct Space {
  pub halves: [Half; 2],
}

#[derive(Debug)]
pub struct Half {
  pub stars: Vec<Star>,
}

#[derive(Debug)]
pub struct Star {
  pub x: u16,
  pub y: u16,
  pub symbol: &'static str,
}
