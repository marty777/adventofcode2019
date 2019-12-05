// day 5 
use std::io;

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let intcodes_str = vec[0].split(","); 
	let mut intcodes: Vec<i64> = Vec::new();
	for code in intcodes_str {
		let temp: i64 = code.parse::<i64>().unwrap();
		intcodes.push(temp);
	}
	let mut op_pos: usize = 0;
	
	loop  {
		let opcode = intcodes[op_pos] % 100;
		let mode1 = ((intcodes[op_pos] - opcode) % 1000)/100;
		let mode2 = ((intcodes[op_pos] - opcode - 100*mode1) % 10000)/1000;
		let _mode3 = ((intcodes[op_pos] - opcode - 100*mode1 - 1000*mode2) % 100000)/10000; // presumably this will be used in future opcodes. Unused at the moment.
		
		match opcode {
			1 => { // add
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				let param3 = intcodes[op_pos + 3];
				intcodes[param3 as usize] = (if mode1 == 1 {param1} else {intcodes[param1 as usize]} ) + (if mode2 == 1 {param2} else {intcodes[param2 as usize]} );
				op_pos += 4;
			},
			2 => { // mul 
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				let param3 = intcodes[op_pos + 3];
				intcodes[param3 as usize] = (if mode1 == 1 {param1} else {intcodes[param1 as usize]} ) * (if mode2 == 1 {param2} else {intcodes[param2 as usize]} );
				op_pos += 4;
			},
			3 => { // input
				let param1 = intcodes[op_pos + 1];
				println!("Enter input: ");
				let mut input = String::new();
				io::stdin().read_line(&mut input).expect("Reading from stdin won't fail");
				let input64:i64 = input.trim().parse::<i64>().unwrap();
				intcodes[param1 as usize] = input64;
				op_pos += 2;
			},
			4 => { // output
				let param1 = intcodes[op_pos + 1];
				println!("Output: {}", intcodes[param1 as usize]);
				op_pos += 2;
			},
			5 => { // jump-if-true
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				if (if mode1 == 1 {param1} else {intcodes[param1 as usize]}) != 0 {
					op_pos = if mode2 == 1 {param2 as usize} else {intcodes[param2 as usize] as usize};
				}
				else {
					op_pos += 3;
				}
			},
			6 => { // jump-if-false
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				if (if mode1 == 1 {param1} else {intcodes[param1 as usize]}) == 0 {
					op_pos = if mode2 == 1 {param2 as usize} else {intcodes[param2 as usize] as usize};
				}
				else {
					op_pos += 3;
				}
			},
			7 => { // lt
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				let param3 = intcodes[op_pos + 3];
				if (if mode1 == 1 {param1} else {intcodes[param1 as usize]}) <  (if mode2 == 1 {param2} else {intcodes[param2 as usize]}) {
					intcodes[param3 as usize] = 1;
				}
				else {
					intcodes[param3 as usize] = 0;
				}
				op_pos += 4;
			},
			8 => { // eq
				let param1 = intcodes[op_pos + 1];
				let param2 = intcodes[op_pos + 2];
				let param3 = intcodes[op_pos + 3];
				if (if mode1 == 1 {param1} else {intcodes[param1 as usize]}) ==  (if mode2 == 1 {param2} else {intcodes[param2 as usize]}) {
					intcodes[param3 as usize] = 1;
				}
				else {
					intcodes[param3 as usize] = 0;
				}
				op_pos += 4;
			},
			99 => {println!("Program complete");break;}
			_ => {println!("Invalid opcode found {}", opcode); break;}
		}
		// println!("op pos {}", op_pos);
		// let op1:usize = intcodes[op_pos + 1];
		// let op2:usize = intcodes[op_pos + 2];
		// let op3:usize = intcodes[op_pos + 3];
		// if intcodes[op_pos] == 1 {
			// intcodes[op3] = intcodes[op1] + intcodes[op2];
		// }
		// else if intcodes[op_pos] == 2 {
			// intcodes[op3] = intcodes[op1] * intcodes[op2];
		// }
		// op_pos += 4;
		
	}
}