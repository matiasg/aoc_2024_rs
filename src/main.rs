mod day_1;
mod day_2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => day_1::main(),
        "2" => day_2::main(),
        &_ => todo!(),
    }
}
