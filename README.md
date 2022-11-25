# sensulator

A rust library for simulating sensor measurement behavior.
You can initialize or update a sensulator with a central
"ideal" value, and the sensulator will subsequently provide
noisy sensor measurements centered around that value.

This library allows you to provide a random number generator (RNG)
that could be based on either unpredictable, truly random behavior
(such as `StdRng` when used with `std`) or predictable, reproducible 
behavior when used with a `SeedableRng`.



## example

See examples:
- [`predict_rng`](examples/predict_rng.rs)
- [`std_rng`](examples/std_rng.rs)


## Testing with quickcheck

A brief quickcheck is run as part of `cargo test`; however, if you want to run more
extensive tests, you can use something like:

```
export QUICKCHECK_TESTS=1000; cargo test -- --nocapture
```


