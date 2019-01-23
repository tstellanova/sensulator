/*
Copyright (c) 2018 Todd Stellanova
LICENSE: See LICENSE file
*/

#![crate_type = "lib"]

extern crate rand;
use rand::distributions::{Normal, Distribution};

/// Standard resolution for sensor measurement values
pub type MeasureVal = f32;


/// This many standard deviations (sigma) is the full error range; typically 3 sigma = 99.7% of values
const STD_DEV_RANGE : MeasureVal  = 3 as MeasureVal;
const ZERO_VAL : MeasureVal=  0 as MeasureVal;

pub struct Sensulator {
  center_value: MeasureVal,
  offset_center_value: MeasureVal,
  relative_err_std_dev: MeasureVal,
  absolute_err_offset: MeasureVal,
  
  simulated_reading_source: Box<rand::distributions::Normal>,
}

impl Sensulator {
  
  pub fn new(ctr_val: MeasureVal, abs_err_range: MeasureVal, rel_err: MeasureVal) -> Sensulator {
    let mut this =  Sensulator {
        center_value: ZERO_VAL,
        offset_center_value: ZERO_VAL,
        relative_err_std_dev: ZERO_VAL,
        absolute_err_offset: ZERO_VAL,
        simulated_reading_source: Box::new(Normal::new(ZERO_VAL as f64, 666 as f64)),
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
    let std_dev = err_range.abs() / STD_DEV_RANGE; //Assumes three standard deviations is full absolute error range
    let abs_err_dist = Normal::new(0.into(), std_dev.into() );
    self.set_absolute_error_offset( abs_err_dist.sample(&mut rand::thread_rng()) as MeasureVal );
  }
  
  /// Set the concrete offset of the simulator's "sensed" measurement from the actual value.
  ///
  /// Generally you should prefer `set_absolute_error_range` instead
  pub fn set_absolute_error_offset(&mut self, err_offset: MeasureVal)  {
    self.absolute_err_offset = err_offset.abs();
  }
  
  /// Set the sensor simulator's relative error: the precision of the sensor.
  pub fn set_relative_error(&mut self, err: MeasureVal) {
    self.relative_err_std_dev = err.abs() / STD_DEV_RANGE;
  }
  
  /// Set the sensor simulator's "actual" value.
  /// This will be adjusted by absolute and relative errors to provide simulated measurement noise.
  pub fn set_center_value(&mut self, val: MeasureVal) {
    self.center_value = val;
    self.offset_center_value = self.center_value + self.absolute_err_offset;
    self.simulated_reading_source = Box::new(Normal::new(self.offset_center_value.into(), self.relative_err_std_dev.into()) );
  }
  
  /// Provide one simulated sensor reading
  pub fn read(&mut self) -> MeasureVal {
    // TODO pin to min / max values ? or accept that low STD_DEV_RANGE means some samples fall outside error range
    self.simulated_reading_source.sample(&mut rand::thread_rng()) as MeasureVal
  }

}


#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
  
  use ::{MeasureVal, Sensulator};

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
    let mut senso = Sensulator::new(CENTER_VAL, ABS_ERR, REL_ERR);

    for _x in 0..10000 {
      let val = senso.read();
      assert!(sample_in_range(val, CENTER_VAL, ABS_ERR, REL_ERR));
    }
    
  }
  
  #[test]
  fn edge_config_values() {
    let abs_err = 0 as MeasureVal;
    let rel_err = -1 as MeasureVal;
    let ctr_val = 0 as MeasureVal;
    
    let mut senso = Sensulator::new(ctr_val, abs_err, rel_err);
    let val = senso.read();
    assert!(sample_in_range(val, ctr_val, abs_err, rel_err));
  }
  
  
  #[test]
  quickcheck! {
      fn check_output_range(abs_err: MeasureVal, rel_err: MeasureVal, ctr_val: MeasureVal) -> bool {
          let mut senso = Sensulator::new(ctr_val, abs_err, rel_err);
          for _count in 0..100 {
            let val = senso.read();
            if !sample_in_range(val, ctr_val, abs_err, rel_err) {
              return false;
            }
          }
          true
      }
  }
  
}


