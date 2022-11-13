// Copyright 2019, Todd Stellanova
// License: see LICENSE file


/// Example using the standard RNG, using std
///
use sensulator::Sensulator;
use rand::SeedableRng;
// use rand::rngs::SmallRng;
use rand::rngs::StdRng;

/// Latitude of Berkeley, California
const HOME_LAT:f32 = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:f32 = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:f32 = 4.5e-5;

fn main() {
  // create a predictable RNG starting with a seed
  // const HAY_SEED: [u8; 32] = [
  //   0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
  //   0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
  //   0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
  //   0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
  // ];
  // let my_rng = rand::rngs::StdRng::from_seed(HAY_SEED);

  // create an unpredictable RNG using std-provided entropy source
  let my_rng = StdRng::from_entropy();
  let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, Box::new(my_rng));
  loop {
    // update the sensor reading and display (requires a mutable sensulator reference)
    println!("new lat: {}", fake_gps_lat.measure());
    // simply display the last measured value (may use an immutable reference)
    println!("old lat: {}", fake_gps_lat.peek());
  }
}

