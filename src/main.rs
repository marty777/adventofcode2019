use std::env;
use std::path::Path;
pub mod utility;
pub mod day1;

const MAXDAY:u32 = 1;

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
	 _=>{usage(); return;}
	}
}

fn usage() {
	println!("Usage:");
	println!("\tcargo run [DAY] [INPUT FILE] ...\n");
	println!("\t[DAY]\t\tThe advent program day to run (between 1 and {})", MAXDAY);
	println!("\t[INPUT FILE]\tThe relative or absolute path to the input file.");
}

pub fn main_test() {
	println!("Hello, world!2");
}
