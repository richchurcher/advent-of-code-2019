#![feature(try_trait)]

use aoc_runner_derive::aoc_lib;

extern crate failure;
extern crate itertools;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub(crate) mod intcode;

aoc_lib! { year = 2019 }
