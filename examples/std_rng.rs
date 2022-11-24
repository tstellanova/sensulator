// Copyright 2022, Todd Stellanova
// License: see LICENSE file


/// Example using the standard RNG, using std
///
use sensulator::{MeasureVal, Sensulator};
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Latitude of Berkeley, California
const HOME_LAT:MeasureVal = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:MeasureVal = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:MeasureVal = 4.5e-5;

fn main() {
  // create an unpredictable RNG using std-provided entropy source
  let mut my_rng = StdRng::from_entropy();
  let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, &mut my_rng);
  loop {
    // update the sensor reading and display (requires a mutable sensulator reference)
    println!("new lat: {}", fake_gps_lat.measure());
    // simply display the last measured value (may use an immutable reference)
    println!("old lat: {}", fake_gps_lat.peek());
  }
}

