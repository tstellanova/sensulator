/*
Copyright (c) 2018 Todd Stellanova
LICENSE: See LICENSE file
*/

use rand::{self};
use rand::distributions::{Normal, Distribution};



pub type MeasureVal = f32;
// type Measurements = Vec<MeasureVal>;
// type MeasureError = MeasureVal;
// type AbsError = MeasureError;
// type AbsErrOffsets = Vec<AbsError>;





pub struct Sensulator {
  center_value: MeasureVal,
  offset_center_value: MeasureVal,
  relative_err_std_dev: MeasureVal,
  absolute_err_offset: MeasureVal,
  simulated_reading_source: Box<rand::distributions::Normal>,
  
}

impl Sensulator {

  pub fn new() -> Sensulator {
    let zeroval =  0 as MeasureVal;
    let this = Sensulator {
        center_value: zeroval,
        offset_center_value: zeroval,
        relative_err_std_dev: zeroval,
        absolute_err_offset: zeroval,
        simulated_reading_source: Box::new(Normal::new(0 as f64, 666 as f64)),
    };

    this
  }
  
  /// Set the range of absolute error: the accuracy of the sensor.
  pub fn set_absolute_error_range(&mut self, err_range: MeasureVal) {
    //absolute error is a range, eg +/- 100 Pascals
    //here we calculate a concrete error offset from the range
    //randomized with a normal distribution
    let std_dev = err_range / (3 as MeasureVal); //Assumes three standard deviations is full absolute error range
    let abs_err_dist = Normal::new(0.into(), std_dev.into() );
    self.set_absolute_error_offset( abs_err_dist.sample(&mut rand::thread_rng()) as MeasureVal );
  }
  
  /// Set the concrete offset of the simulator's "sensed" measurement from the actual value.
  ///
  /// Generally you should prefer `set_absolute_error_range` instead
  pub fn set_absolute_error_offset(&mut self, err_offset: MeasureVal)  {
    self.absolute_err_offset = err_offset;
  }
  
  /// Set the sensor simulator's relative error: the precision of the sensor.
  pub fn set_relative_error(&mut self, err: MeasureVal) {
    self.relative_err_std_dev = err / (3 as MeasureVal);// Assumes three standard deviations is full rel error range
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
    let cur_sample = self.simulated_reading_source.sample(&mut rand::thread_rng()) as MeasureVal;
    cur_sample
  }
  
  
  // fn calc_average_val(samples: &Measurements) -> MeasureVal {
  //   let sum: MeasureVal = samples.iter().sum();
  //   let result = sum / (samples.len() as MeasureVal);
  //   result
  // }
  //
  // fn init_error_offsets(offs: &mut AbsErrOffsets){
  //   let abs_err_std_dev = ABS_ERR / (NUM_SENSORS as MeasureVal).sqrt();
  //   let abs_err_dist = Normal::new(0.into(), abs_err_std_dev.into());
  //
  //   for mut x in offs {
  //     *x = abs_err_dist.sample(&mut rand::thread_rng()) as AbsError;
  //   }
  // }
}

