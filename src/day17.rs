// day 17

fn to_iobuffer(iobuff:&mut super::utility::IOBuffer, input:&str) {
	for c in input.chars() {
		let val = c as u8;
		(*iobuff).buff.push(val as i64);
		(*iobuff).write_pos += 1;
	}
}

fn determine_turn(dir:u8, desired_dir:u8)->String {
	let mut change = (desired_dir as i64) - (dir as i64);
	if change < 0 {
		change += 4;
	}
	change = change % 4;
	if change == 1 {
		return String::from("R");
	}
	else if change == 3 {
		return String::from("L");
	}
	else {
		return String::from("ERROR");
	}
}

fn grid_has_scaffold(grid:&mut Vec<u8>, x:i64, y:i64, width:usize, height:usize)->bool{
	if x < 0 || x >= (width as i64) {
		return false;
	}
	if y < 0 || y >= (height as i64) {
		return false;
	}
	if (*grid)[((y*(width as i64)) + x) as usize] == 35 {
		return true;
	}
	return false;
}

fn run_bot(prog:&mut super::utility::IntcodeProgram, part_a:bool)->i64 {
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	
	let mut grid:Vec<u8> = Vec::new();
	
	
	in_buffer.buff.clear();
	in_buffer.read_pos = 0;
	in_buffer.write_pos = 0;
	out_buffer.buff.clear();
	out_buffer.read_pos = 0;
	out_buffer.write_pos = 0;
	
	if part_a {
		super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		
		for i in 0..out_buffer.buff.len() {
			grid.push(out_buffer.buff[i] as u8);
			print!("{}", (out_buffer.buff[i] as u8) as char)
		}
		
		let mut width:usize = 0;
		let mut height:usize = 0;
		for i in 0..grid.len() {
			if grid[i] == 10 { // newline 
				width = i+1;
				height = grid.len()/width;
				break;
			}
		}
		let mut intersection_sum = 0;
		for y in 1..height - 1 {
			for x in 1..width - 1{
				if 	grid[(y*width) + x] == 35 && 
					grid[(y*width) + x + 1] == 35 && 
					grid[(y*width) + x - 1] == 35 && 
					grid[(y+1)*width + x] == 35 && 
					grid[(y-1)*width + x] == 35 {
						intersection_sum += x * y;
						
				}
			}
		}
		
		println!("Result A: {}", intersection_sum);
	}
	else {
		// get the grid 
		super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);

		for i in 0..out_buffer.buff.len() {
			grid.push(out_buffer.buff[i] as u8);
		}
		let mut width:usize = 0;
		let mut height:usize = 0;
		for i in 0..grid.len() {
			if grid[i] == 10 { // newline 
				width = i+1;
				height = grid.len()/width;
				break;
			}
		}
		
		// get initial position
		let mut dir = 0; // north
		let mut pos_x:i64 = 0;
		let mut pos_y:i64 = 0;
		for y in 1..height - 1 {
			for x in 1..width - 1{
				if grid[(y*width) + x] != 10 && grid[(y*width) + x] != 46 { // not newline or period
					if grid[(y*width) + x] == 94 { // up
						pos_x = x as i64;
						pos_y = y as i64;
						dir = 0;
						break;
					}
					else if grid[(y*width) + x] == 62 { // R 
						pos_x = x as i64;
						pos_y = y as i64;
						dir = 1;
					}
					else if grid[(y*width) + x] == 118 { // D 
						pos_x = x as i64;
						pos_y = y as i64;
						dir = 2;
					}
					else if grid[(y*width) + x] == 60 { // L 
						pos_x = x as i64;
						pos_y = y as i64;
						dir = 3;
					}
				}
			}
		}
		
		if pos_x == 0 && pos_y == 0 {
			println!("Couldn't find robot");
			return 0;
		}
		
		// go forward until you can't, then turn, never stop at intersections
		let mut prev_x = pos_x;
		let mut prev_y = pos_y;
		
		let mut instructions:Vec<String> = Vec::new();
		
		loop {
			// determine direction to turn.
			let mut up_available = false;
			let mut left_available = false;
			let mut right_available = false;
			let mut down_available = false;
			if grid_has_scaffold(&mut grid, pos_x, pos_y-1, width, height) && prev_y != pos_y - 1{
				up_available = true;
			}
			if grid_has_scaffold(&mut grid, pos_x-1, pos_y, width, height) && prev_x != pos_x - 1{
				left_available = true;
			}
			if grid_has_scaffold(&mut grid, pos_x+1, pos_y, width, height) && prev_x != pos_x + 1{
				right_available = true;
			}
			if grid_has_scaffold(&mut grid, pos_x, pos_y+1, width, height) && prev_y != pos_y + 1{
				down_available = true;
			}
			let mut count_available = 0;
			if up_available {
				count_available += 1;
			}
			if down_available {
				count_available += 1;
			}
			if left_available {
				count_available += 1;
			}
			if right_available {
				count_available += 1;
			}
			if count_available > 1 {
				println!("Too many options available {}", count_available);
				return 0;
			}
			else if count_available == 0 {
				break;
			}
			
			// determine turn
			if up_available {
				instructions.push(determine_turn(dir, 0));
				dir = 0;
			}
			else if right_available {
				instructions.push(determine_turn(dir, 1));
				dir = 1;
			}
			else if down_available {
				instructions.push(determine_turn(dir, 2));
				dir = 2;
			}
			else if left_available {
				instructions.push(determine_turn(dir, 3));
				dir = 3;
			}
			
			// go forward until you can't
			let mut forward_count = 0;
			if dir == 0 {
				while grid_has_scaffold(&mut grid, pos_x, pos_y-1, width, height) {
					prev_x = pos_x;
					prev_y = pos_y;
					pos_y -= 1;
					forward_count += 1;
				}
			}
			else if dir == 1 {
				while grid_has_scaffold(&mut grid, pos_x+1, pos_y, width, height) {
					prev_x = pos_x;
					prev_y = pos_y;
					pos_x += 1;
					forward_count += 1;
				}
			}
			else if dir == 2 {
				while grid_has_scaffold(&mut grid, pos_x, pos_y+1, width, height) {
					prev_x = pos_x;
					prev_y = pos_y;
					pos_y += 1;
					forward_count += 1;
				}
			}
			else if dir == 3 {
				while grid_has_scaffold(&mut grid, pos_x-1, pos_y, width, height) {
					prev_x = pos_x;
					prev_y = pos_y;
					pos_x -= 1;
					forward_count += 1;
				}
			}
			instructions.push(forward_count.to_string());		
		}
		
		// process the instruction list into program chunks.
		// My grasp of string manipulation is a bit weak in rust.
		let mut instructions_temp:String = String::new();
		for i in 0..instructions.len() {
			if i == instructions.len() - 1 {
				instructions_temp.insert_str(instructions_temp.len(), instructions[i].as_str());
			}
			else {
				instructions_temp.insert_str(instructions_temp.len(), instructions[i].as_str());
				instructions_temp.insert(instructions_temp.len(), ',');
			}
		}
		
		let mut comma_count = 0;
		let mut a_end = 0;
		let instruction_bytes = instructions_temp.as_bytes();
		for i in 0..instruction_bytes.len() {
			if i > 20 {
				break;
			}
			if instruction_bytes[i] == 44 { 
				comma_count += 1;
				if comma_count % 2 == 0 {
					a_end = i;
				}
			}
		}
		let mut a = instructions_temp[0..a_end].to_string();
		instructions_temp = instructions_temp.replace(a.as_str(), "A");
		
		let mut b_start = 0;
		let mut b_end = 0;
		let instruction_bytes2 = instructions_temp.as_bytes();
		let mut on_a = true;
		comma_count = 0;
		for i in 0..instruction_bytes2.len() {
			// skip any As.
			if on_a && instruction_bytes2[i] != 44 && instruction_bytes2[i] != 65 {
				b_start = i;
				on_a = false;
			}
			if !on_a {
				if i - b_start > 20 || instruction_bytes2[i] == 65 {
					break;
				}
				if instruction_bytes2[i] == 44 { 
					comma_count += 1;
					if comma_count % 2 == 0 {
						b_end = i;
					}
				}
			}
		}
		let mut b = instructions_temp[b_start..b_end].to_string();
		instructions_temp = instructions_temp.replace(b.as_str(), "B");
		
		let mut c_start = 0;
		let mut c_end = 0;
		let instruction_bytes3 = instructions_temp.as_bytes();
		let mut on_ab = true;
		comma_count = 0;
		for i in 0..instruction_bytes3.len() {
			// skip any As.
			if on_ab && instruction_bytes3[i] != 44 && instruction_bytes3[i] != 65 && instruction_bytes3[i] != 66 {
				c_start = i;
				on_ab = false;
			}
			if !on_ab {
				if i - c_start > 20 || instruction_bytes3[i] == 65 || instruction_bytes3[i] == 66 {
					break;
				}
				if instruction_bytes3[i] == 44 { 
					comma_count += 1;
					if comma_count % 2 == 0 {
						c_end = i;
					}
				}
			}
		}
		let mut c = instructions_temp[c_start..c_end].to_string();
		instructions_temp = instructions_temp.replace(c.as_str(), "C");
		
		if instructions_temp.contains("L") || instructions_temp.contains("R") {
			println!("Unable to construct instruction strings. Bailing...");
			return 0;
		}
		
		instructions_temp.push('\n');
		a.push('\n');
		b.push('\n');
		c.push('\n');
		
		to_iobuffer(&mut in_buffer, instructions_temp.as_str());
		to_iobuffer(&mut in_buffer, a.as_str());
		to_iobuffer(&mut in_buffer, b.as_str());
		to_iobuffer(&mut in_buffer, c.as_str());
		to_iobuffer(&mut in_buffer, "n\n"); // camera
		super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		println!("Result B: {}", out_buffer.buff[out_buffer.buff.len() - 1]);

	}
	return 0;
}



pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let intcodes_str:Vec<&str> = vec[0].split(",").collect(); 
	let mut prog_a:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	let mut prog_b:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	prog_a.mem.reserve(intcodes_str.len());
	prog_b.mem.reserve(intcodes_str.len());
	for code in intcodes_str {
		let temp: i64 = code.parse::<i64>().unwrap();
		prog_a.mem.push(temp);
		prog_b.mem.push(temp);
	}
	
	
	run_bot(&mut prog_a, true);
	
	prog_b.mem[0] = 2;
	run_bot(&mut prog_b, false);
	

}