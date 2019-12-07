// day 7 

pub fn execute_amp(intcodes: &mut Vec<i64>, input_signal:i64, phase:i64) -> i64 {
	let mut exit = false;
	let mut input_count = 0;
	let mut op_pos:usize = 0;
	return execute_amp2(intcodes, input_signal, phase, &mut input_count, &mut op_pos, &mut exit);
}

pub fn execute_amp2(intcodes: &mut Vec<i64>, input_signal:i64, phase:i64, input_count:&mut u64, pos: &mut usize, exit:&mut bool) -> i64 {
	let mut op_pos: usize = *pos;
	let mut output:i64 = 0;
	let mut input_signal_used = false;
	let mut exit_found = false;
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
				
				let mut input64:i64 = 0;
				if *input_count == 0 {
					input64 = phase;
				}
				else if *input_count >= 1 {
					if !input_signal_used {
						input64 = input_signal;
						input_signal_used = true;
						//println!("Input {}", input_signal);
					}
					else {
						exit_found = false;
						break;
					}
				}
				
				*input_count+=1;
				intcodes[param1 as usize] = input64;
				op_pos += 2;
			},
			4 => { // output
				let param1 = intcodes[op_pos + 1];
				output = intcodes[param1 as usize];
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
			99 => {exit_found = true; break;}
			_ => {println!("Invalid opcode found {}", opcode); break;}
		}
	}
	*pos = op_pos;
	*exit = exit_found;
	return output;
} 

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let intcodes_str = vec[0].split(","); 
	let mut intcodes: Vec<i64> = Vec::new();
	for code in intcodes_str {
		let temp: i64 = code.parse::<i64>().unwrap();
		intcodes.push(temp);
	}
	
	let mut max_output = 0;
	let mut last_output:i64;
	for a in 0..5 {
		for b in 0..5 {
			if b == a {
				continue;
			}
			for c in 0..5 {
				if c==a || c==b {
					continue;
				}
				for d in 0..5 {
					if d == a || d == b || d == c {
						continue;
					}
					for e in 0..5 {
						
						if e == a || e == b || e == c || e == d {
							continue;
						}
						last_output = 0;
						for i in 0..5 {
							let mut intcodes_temp = intcodes.to_vec();
							let phase:i64;
							match i {
								0=>phase=a,
								1=>phase=b,
								2=>phase=c,
								3=>phase=d,
								4=>phase=e,
								_=>{println!("something bad happened"); return;}
							}
							
							last_output = execute_amp(&mut intcodes_temp, last_output, phase);
							
						}
						if last_output > max_output  {
							max_output = last_output;
						}
					}
				}
			}
		}
	}
	
	println!("Result A: {}", max_output);
	
	max_output = 0;
	for a in 5..10 {
		for b in 5..10 {
			if b == a {
				continue;
			}
			for c in 5..10 {
				if c==a || c==b {
					continue;
				}
				for d in 5..10 {
					if d == a || d == b || d == c {
						continue;
					}
					for e in 5..10 {
						
						if e == a || e == b || e == c || e == d {
							continue;
						}
						last_output = 0;
						let mut intcodes_a = intcodes.to_vec();
						let mut intcodes_b = intcodes.to_vec();
						let mut intcodes_c = intcodes.to_vec();
						let mut intcodes_d = intcodes.to_vec();
						let mut intcodes_e = intcodes.to_vec();
						let mut exit = false;
						
						let mut input_count_a = 0;
						let mut input_count_b = 0;
						let mut input_count_c = 0;
						let mut input_count_d = 0;
						let mut input_count_e = 0;
						
						let mut	pos_a:usize = 0;
						let mut	pos_b:usize = 0;
						let mut	pos_c:usize = 0;
						let mut	pos_d:usize = 0;
						let mut	pos_e:usize = 0;
						
						while !exit {
							last_output = execute_amp2(&mut intcodes_a, last_output, a, &mut input_count_a, &mut pos_a, &mut exit);
							last_output = execute_amp2(&mut intcodes_b, last_output, b, &mut input_count_b, &mut pos_b, &mut exit);
							last_output = execute_amp2(&mut intcodes_c, last_output, c, &mut input_count_c, &mut pos_c, &mut exit);
							last_output = execute_amp2(&mut intcodes_d, last_output, d, &mut input_count_d, &mut pos_d, &mut exit);
							last_output = execute_amp2(&mut intcodes_e, last_output, e, &mut input_count_e, &mut pos_e, &mut exit);
							if exit {
								if last_output > max_output {
									//println!("New high configuration {} {} {} {} {} : {}", a, b, c, d, e, last_output);
									max_output = last_output;
									break;
								}
							}
						}
						
						
					}
				}
			}
		}
	}
	
	println!("Result B: {}", max_output);
}