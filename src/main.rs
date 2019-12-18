use std::env;
use std::path::Path;
pub mod utility;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;

const MAXDAY:u32 = 18;

fn main() {
	let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
		usage();
		return;
	}
	let day:u32 = match args[1].trim().parse() {
		Ok(num) => num,
		Err(_) => {usage(); return;},
	};
	if day < 1 || day > MAXDAY  {
		usage();
		return;
	}
	
	if !Path::new(&args[2]).is_file() {
		println!("File {} could not be found", args[2]);
		usage();
		return;
	}
	
	match day {
	 1=>day1::run(&args[2]),
	 2=>day2::run(&args[2]),
	 3=>day3::run(&args[2]),
	 4=>day4::run(&args[2]),
	 5=>day5::run(&args[2]),
	 6=>day6::run(&args[2]),
	 7=>day7::run(&args[2]),
	 8=>day8::run(&args[2]),
	 9=>day9::run(&args[2]),
	 10=>day10::run(&args[2]),
	 11=>day11::run(&args[2]),
	 12=>day12::run(&args[2]),
	 13=>day13::run(&args[2]),
	 14=>day14::run(&args[2]),
	 15=>day15::run(&args[2]),
	 16=>day16::run(&args[2]),
	 17=>day17::run(&args[2]),
	 18=>day18::run(&args[2]),
	 _=>{usage(); return;}
	}
}

fn usage() {
	println!("Usage:");
	println!("\tcargo run [DAY] [INPUT FILE] ...\n");
	println!("\t[DAY]\t\tThe advent program day to run (between 1 and {})", MAXDAY);
	println!("\t[INPUT FILE]\tThe relative or absolute path to the input file.");
}
