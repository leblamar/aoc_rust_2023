use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;


fn main() {
  env::set_var("RUST_BACKTRACE", "1");
  println!("Let's start Advent of Code 2023 !!!");

  day1::main();
  day2::main();
  day3::main();
  day4::main();
  day5::main();
}
