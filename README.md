# sensulator

A rust library for simulating sensor measurement behavior.
You can initialize or update a sensulator with a central
"ideal" value, and the sensulator will subsequently provide
noisy sensor measurements centered around that value.

## example

See [`main.rs`](src/main.rs)

```
  use sensulator::Sensulator;
  
  /// Latitude of Berkeley, California
  const HOME_LAT:f32 = 37.8716;
  /// Absolute error of a typical GPS sensor (degrees)
  const GPS_HORIZ_ABS_ERROR:f32 = 2e-6;
  /// Relative error of a typical GPS sensor (degrees)
  const GPS_HORIZ_REL_ERROR:f32 = 4.5e-5;
  
  let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR);
  loop {
    // update the sensor reading and display (requires a mutable sensulator reference)
    println!("new lat: {}", fake_gps_lat.measure());
    // simply display the last measured value (may use an immutable reference)
    println!("old lat: {}", fake_gps_lat.peek());
  }
```


## Testing with quickcheck

A brief quickcheck is run as part of `cargo test`; however, if you want to run more
extensive tests, you can use something like:

```
export QUICKCHECK_TESTS=1000000; cargo test -- --nocapture
```


