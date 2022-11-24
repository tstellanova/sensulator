// Copyright 2022, Todd Stellanova
// License: see LICENSE file


/// Example using a predictable, seeded PRNG
///
use sensulator::{MeasureVal, Sensulator};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Latitude of Berkeley, California
const HOME_LAT:MeasureVal = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:MeasureVal = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:MeasureVal = 4.5e-5;

fn main() {
  // create a platform-independent predictable PRNG starting with a seed
  const HAY_SEED: [u8; 32] = [
    0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
    0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
    0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
    0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
  ];
  let mut my_rng = ChaCha8Rng::from_seed(HAY_SEED);

  let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, &mut my_rng);
  let  first_val = fake_gps_lat.measure();
  let second_val = fake_gps_lat.measure();
  println!("first: {} second: {}", first_val, second_val);

  assert_eq!(first_val,  37.871628);
  assert_eq!(second_val, 37.87161);

}

