#![no_main]
#![no_std]

use funtimes as _; // global logger + panicking-behavior + memory layout
use sensulator::Sensulator;
use rand_core::{SeedableRng};
use rand::rngs::SmallRng;


/// Latitude of Berkeley, California
const HOME_LAT:f32 = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:f32 = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:f32 = 4.5e-5;


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("chunky!");

    // create a predictable RNG starting with a seed
    const HAY_SEED: [u8; 16] = [
        0xFE, 0xED, 0xAB, 0xBA, 0xDE, 0xAD, 0xBE, 0xEF,
        0xFE, 0xED, 0xAB, 0xBA, 0xDE, 0xAD, 0xBE, 0xEF,
        //0xFE, 0xED, 0xAB, 0xBA, 0xDE, 0xAD, 0xBE, 0xEF,
        //0xFE, 0xED, 0xAB, 0xBA, 0xDE, 0xAD, 0xBE, 0xEF,
    ];
    let mut my_rng = SmallRng::from_seed(HAY_SEED);

    let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, &mut my_rng);
    loop {
        // update the sensor reading and display (requires a mutable sensulator reference)
        defmt::println!("new lat: {}", fake_gps_lat.measure());
        // simply display the last measured value (may use an immutable reference)
        defmt::println!("old lat: {}", fake_gps_lat.peek());
    }

}