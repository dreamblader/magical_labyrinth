use std::env;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let champion_num: Result<u8, ParseIntError> = args[1].parse::<u8>();

    match champion_num {
        Result::Ok(num) => println!("Number: {}", num),
        Result::Err(_) => println!("No valid number of champions to be generated was specified"),
    }
}
