use std::env;

mod day1;
mod day2;


fn main() {
  env::set_var("RUST_BACKTRACE", "1");
  println!("Let's start Advent of Code 2023 !!!");

  day1::main();
  day2::main();
}
