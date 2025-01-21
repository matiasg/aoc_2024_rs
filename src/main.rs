mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::env;

fn main() {
    let mains = [day_1::main, day_2::main, day_3::main, day_4::main];

    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("can't understand day to run");
    match day {
        i if i > mains.len() || i < 1 => panic!("No day {day}"),
        i => mains[i - 1](),
    };
}
