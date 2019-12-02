// day 2
pub fn run(file_path:&str) {
		let vec = super::utility::util_fread(file_path);
		let intcodes_str = vec[0].split(",");
		let mut intcodes: Vec<usize> = Vec::new();
		let mut intcodes_b: Vec<usize> = Vec::new();
		for code in intcodes_str {
			let temp: usize = code.parse::<usize>().unwrap();
			intcodes.push(temp);
			intcodes_b.push(temp);
		}
		let mut op_pos: usize = 0;
		// starting condition modification
		intcodes[1] = 12;
		intcodes[2] = 2;
		while intcodes[op_pos] != 99  {
			let op1:usize = intcodes[op_pos + 1];
			let op2:usize = intcodes[op_pos + 2];
			let op3:usize = intcodes[op_pos + 3];
			if intcodes[op_pos] == 1 {
				intcodes[op3] = intcodes[op1] + intcodes[op2];
			}
			else if intcodes[op_pos] == 2 {
				intcodes[op3] = intcodes[op1] * intcodes[op2];
			}
			op_pos += 4;
		}
		
		println!("Part A result: {}", intcodes[0]);
		
		let mut done:bool = false;
		let desired_output = 19690720;
		let mut result_b = 0;
		for noun in 0..99 {
			if done {
				break;
			}
			for verb in 0..99 {
				let mut intcodes_test = intcodes_b.to_vec();
				intcodes_test[1] = noun;
				intcodes_test[2] = verb;
				let mut op_pos = 0;
				while intcodes_test[op_pos] != 99 {
					let op1:usize = intcodes_test[op_pos + 1];
					let op2:usize = intcodes_test[op_pos + 2];
					let op3:usize = intcodes_test[op_pos + 3];
					if intcodes_test[op_pos] == 1 {
						intcodes_test[op3] = intcodes_test[op1] + intcodes_test[op2];
					}
					else if intcodes_test[op_pos] == 2 {
						intcodes_test[op3] = intcodes_test[op1] * intcodes_test[op2];
					}
					op_pos += 4;
				}
				if intcodes_test[0] == desired_output {
					done = true;
					result_b = (100*noun) + verb;
					break;
				}
			}
		}
		if !done {
			println!{"Error, could not finish"};
		}
		else {
			println!("Part B result: {} for desired output {}", result_b, desired_output);
		}
		
		
		
		
}