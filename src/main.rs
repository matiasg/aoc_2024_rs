mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;

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
    ];

    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("can't understand day to run");
    match day {
        i if i > mains.len() || i < 1 => panic!("No day {day}"),
        i => mains[i - 1](),
    };
}
