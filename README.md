# sensulator

A rust library for simulating sensor measurement behavior.

## example

```
  use sensulator::Sensulator;
  
  let mut fake_gps_lat = Sensulator::new(HOME_LAT, 1e-3, 1e-6);
  loop {
    println!("lat: {}", fake_gps_lat.read());
  }
```

## Testing with quickcheck

```
export QUICKCHECK_TESTS=1000000; cargo test -- --nocapture
```
