mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
pub mod utils;

use std::env;

fn main() {
    let mains = [
        day_1::main,
        day_2::main,
        day_3::main,
        day_4::main,
        day_5::main,
        day_6::main,
        day_7::main,
        day_8::main,
        day_9::main,
        day_10::main,
        day_11::main,
        day_12::main,
        day_13::main,
        day_14::main,
        day_15::main,
        day_16::main,
        day_17::main,
        day_18::main,
        day_19::main,
        day_20::main,
        day_21::main,
        day_22::main,
    ];

    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("can't understand day to run");
    match day {
        i if i > mains.len() || i < 1 => panic!("No day {day}"),
        i => mains[i - 1](),
    };
}
