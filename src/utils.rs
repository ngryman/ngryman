use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::Rng;

pub fn random_range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}
