/*
Copyright (c) 2018 Todd Stellanova
LICENSE: See LICENSE file
*/

#![crate_type = "lib"]

extern crate rand;

pub mod sensulator;



#[cfg(test)]
mod tests {
  
  use sensulator::{MeasureVal, Sensulator};

  const REL_ERR : MeasureVal  = 12 as MeasureVal;
  const ABS_ERR : MeasureVal = 100 as MeasureVal;
  const CENTER_VAL: MeasureVal = 101325.0 as MeasureVal; // Average air pressure in Pascals
  
  
  #[test]
  fn init_with_values() {
    let mut senso = Sensulator::new();
    senso.set_absolute_error_range(ABS_ERR);
    senso.set_relative_error(REL_ERR);
    senso.set_center_value(CENTER_VAL);
    
    // Verify that sample readings are within the min and max range defined by
    // absolute and relative errors.
    let min_allowed = CENTER_VAL - ABS_ERR - REL_ERR;
    let max_allowed = CENTER_VAL + ABS_ERR + REL_ERR;
    for _x in 0..10000 {
      let val = senso.read();
      //println!("{} {} {}",min_allowed, val, max_allowed);
      assert!(val >= min_allowed);
      assert!(val <= max_allowed);
    }
    
  }
  
}


