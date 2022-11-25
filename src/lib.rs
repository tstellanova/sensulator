/*
Copyright (c) 2022 Todd Stellanova
LICENSE: See LICENSE file
*/

#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]

use rand_core::RngCore;
use rand_distr::Distribution;
use rand_distr::num_traits::Float;

/// Standard resolution for sensor measurement values
pub type MeasureVal = f32;

/// This many standard deviations (sigma) is the full error range; typically 3 sigma = 99.7% of values
pub const STD_DEV_RANGE : MeasureVal  = 3 as MeasureVal;
const ZERO_VAL : MeasureVal=  0 as MeasureVal;

pub struct Sensulator<'a, T:RngCore>  {
  center_value: MeasureVal,
  offset_center_value: MeasureVal,
  relative_err_std_dev: MeasureVal,
  absolute_err_offset: MeasureVal,
  last_measured_value: MeasureVal,
  simulated_reading_source: rand_distr::Normal<MeasureVal>,
  local_rng: &'a mut T,
}

impl<T:RngCore> Sensulator<'_, T> {


  /// Initialize an instance with
  /// - `ctr_vl` : The value that an ideal sensor would measure on every measurement.
  /// - `abs_err_range` : The accuracy of the sensor.
  /// - `rel_err`: The precision of the sensor.
  /// - `rng`: A random number generator to be used for simulation
  ///
  pub fn new(ctr_val: MeasureVal, abs_err_range: MeasureVal, rel_err: MeasureVal, rng: &mut T) -> Sensulator<T> {

    // let new_source = rand_distr::Normal::new(ctr_val.into(), self.relative_err_std_dev.into()).unwrap();
    let tmp_source = rand_distr::Normal::new(ZERO_VAL as f32, 666 as f32).unwrap();
    let mut this =  Sensulator {
        center_value: ZERO_VAL,
        offset_center_value: ZERO_VAL,
        relative_err_std_dev: ZERO_VAL,
        absolute_err_offset: ZERO_VAL,
        simulated_reading_source: tmp_source,
	    last_measured_value: MeasureVal::NAN,
        local_rng: rng,
    };
    this.set_absolute_error_range(abs_err_range);
    this.set_relative_error(rel_err);
    this.set_center_value(ctr_val);
    this
  }


  /// Set the range of absolute error: the accuracy of the sensor.
  pub fn set_absolute_error_range(&mut self, err_range: MeasureVal) {
    //absolute error is a range, eg +/- 100 Pascals
    //here we calculate a concrete error offset from the range
    //randomized with a normal distribution
    let err_range_raw = if err_range.is_infinite() || err_range.is_nan() {
      0 as MeasureVal
    }
    else {
      err_range
    };
    // let err_abs = if err_range < 0 { -err_range } else { err_range };
    let std_dev = err_range_raw.abs() / STD_DEV_RANGE; //Assumes three standard deviations is full absolute error range
    let abs_err_dist = rand_distr::Normal::<MeasureVal>::new(0f32.into(), std_dev.into() );
    let err_off = abs_err_dist.expect("local_rng failed").sample(&mut self.local_rng) as MeasureVal;
    //this is typically only invoked once, at setup time
    self.set_absolute_error_offset( err_off);
  }

  /// Set the concrete offset of the simulator's "sensed" measurement from the actual value.
  ///
  /// Generally you should prefer `set_absolute_error_range` instead
  pub fn set_absolute_error_offset(&mut self, err_offset: MeasureVal)  {
    self.absolute_err_offset = err_offset.abs();
  }

  /// Set the sensor simulator's relative error: the precision of the sensor.
  pub fn set_relative_error(&mut self, rel_err: MeasureVal) {
    let rel_err_raw = if rel_err.is_infinite() || rel_err.is_nan() {
      0 as MeasureVal
    }
    else {
      rel_err
    };
    self.relative_err_std_dev = rel_err_raw.abs() / STD_DEV_RANGE;
  }

  /// Set the sensor simulator's "ideal" value.
  /// This will be adjusted by absolute and relative errors to provide simulated measurement noise.
  pub fn set_center_value(&mut self, val: MeasureVal) {
    self.center_value = val;
    self.offset_center_value = self.center_value + self.absolute_err_offset;
    let new_source = rand_distr::Normal::new(self.offset_center_value.into(),
                                             self.relative_err_std_dev.into()).unwrap();
    self.simulated_reading_source = new_source;
  }

  /// Take a new measurement. This method updates the measured value.
  pub fn measure(&mut self) -> MeasureVal {
    // TODO pin to min / max values ? or accept that low STD_DEV_RANGE means some samples fall outside error range
    self.last_measured_value = self.simulated_reading_source.sample(&mut self.local_rng) as MeasureVal;
    self.last_measured_value
  }

 /// Peek at the last measured value.  This does not update the measured value.
  pub fn peek(&self) -> MeasureVal {
    self.last_measured_value
  }
}


#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand_core;
// extern crate rand;

#[cfg(test)]
mod tests {

  use super::*;
  use rand::rngs::StdRng;
  use crate::rand_core::SeedableRng;
  // extern crate std;


  const REL_ERR : MeasureVal  = 12 as MeasureVal;
  const ABS_ERR : MeasureVal = 100 as MeasureVal;
  const CENTER_VAL: MeasureVal = 101325 as MeasureVal;
  /// How far outside the error range we allow rare outlier samples
  const ERR_RANGE_ALLOWANCE: MeasureVal = 2 as MeasureVal;


  /// Verify that sample readings are within the min and max range defined by absolute and relative errors.
  fn sample_in_range(sample: MeasureVal, ctr_val: MeasureVal, abs_err: MeasureVal, rel_err: MeasureVal) -> bool {
    let tru_abs_err = abs_err.abs() * ERR_RANGE_ALLOWANCE;
    let tru_rel_err = rel_err.abs() * ERR_RANGE_ALLOWANCE;
    let min_allowed = ctr_val - tru_abs_err - tru_rel_err;
    let max_allowed = ctr_val + tru_abs_err + tru_rel_err;

    if (sample >= min_allowed) && (sample <= max_allowed) {
      return true;
    }
    else {
      println!("min: {} val: {} max: {}" , min_allowed, sample, max_allowed);
      return false;
    }
  }

  #[test]
  fn ordinary_config_values() {
    let mut my_rng = StdRng::from_entropy();
    let mut senso = Sensulator::new( CENTER_VAL, ABS_ERR, REL_ERR, &mut my_rng);

    for _x in 0..10000 {
      let val = senso.measure();
      assert!(sample_in_range(val, CENTER_VAL, ABS_ERR, REL_ERR));
    }
  }
  #[test]
  fn test_peek_matches_measure() {
    let mut my_rng = StdRng::from_entropy();
    let mut senso = Sensulator::new(CENTER_VAL, ABS_ERR, REL_ERR, &mut my_rng);

    for _x in 0..10000 {
      let val = senso.measure();
      assert_eq!(val, senso.peek());
    }
  }

  #[test]
  fn edge_config_values() {
    let abs_err = 0 as MeasureVal;
    let rel_err = -1 as MeasureVal;
    let ctr_val = 0 as MeasureVal;

    let mut my_rng = StdRng::from_entropy();
    let mut senso = Sensulator::new(ctr_val, abs_err, rel_err, &mut my_rng);
    let val = senso.measure();
    assert!(sample_in_range(val, ctr_val, abs_err, rel_err));
  }

  quickcheck! {
      fn check_output_range(abs_err: MeasureVal, rel_err: MeasureVal, ctr_val: MeasureVal) -> bool {
          let mut my_rng = StdRng::from_entropy();
          let mut senso = Sensulator::new(ctr_val, abs_err, rel_err, &mut my_rng);
          for _count in 0..1000 {
            let val = senso.measure();
            if !sample_in_range(val, ctr_val, abs_err, rel_err) {
              return false;
            }
          }
          true
      }
  }
  
}


