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

const MAXDAY:u32 = 8;

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
	
	//let filePath1= args[2];
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
	 _=>{usage(); return;}
	}
}

fn usage() {
	println!("Usage:");
	println!("\tcargo run [DAY] [INPUT FILE] ...\n");
	println!("\t[DAY]\t\tThe advent program day to run (between 1 and {})", MAXDAY);
	println!("\t[INPUT FILE]\tThe relative or absolute path to the input file.");
}
