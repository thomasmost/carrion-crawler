use std::fs;

use crate::model::Data;

mod model;
pub mod rule;

pub fn read_config() -> Data {
  // Variable that holds the filename as a `&str`.
  let filename = "carrion.toml";

  // Read the contents of the file using a `match` block
  // to return the `data: Ok(c)` as a `String`
  // or handle any `errors: Err(_)`.
  let contents = match fs::read_to_string(filename) {
    // If successful return the files text as `contents`.
    // `c` is a local variable.
    Ok(c) => c,
    // Handle the `error` case.
    Err(_) => {
      // Write `msg` to `stderr`.
      println!("Could not read file `{}`", filename);
      panic!("Could not read file `{}`", filename);
    }
  };

  // Use a `match` block to return the
  // file `contents` as a `Data struct: Ok(d)`
  // or handle any `errors: Err(_)`.
  let data: Data = match toml::from_str(&contents) {
    // If successful, return data as `Data` struct.
    // `d` is a local variable.
    Ok(d) => d,
    // Handle the `error` case.
    Err(e) => {
      // Write `msg` to `stderr`.
      println!("Unable to load data from `{}`", filename);
      panic!("{:?}", e);
    }
  };

  data
}
