// day 19

fn to_iobuffer(iobuff:&mut super::utility::IOBuffer, input:&str) {
	for c in input.chars() {
		let val = c as u8;
		(*iobuff).buff.push(val as i64);
		(*iobuff).write_pos += 1;
	}
}

fn get_coord(prog:&mut super::utility::IntcodeProgram, x:i64, y:i64)->i64 {
	let mut exit:bool = false;
	let mut prog2 =  super::utility::IntcodeProgram{mem:Vec::new(), pos:prog.pos, relative_base:prog.relative_base};
	for i in 0..prog.mem.len() {
		prog2.mem.push(prog.mem[i]);
	}
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	in_buffer.buff.push(x);
	in_buffer.buff.push(y);
	super::utility::intcode_execute(&mut prog2, &mut in_buffer, &mut out_buffer, &mut exit);
	return out_buffer.buff[0];
}

fn run_probe(prog:&mut super::utility::IntcodeProgram, part_a:bool) {
	let width = 50;
	let height = 50;
	
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut grid:Vec<u8> = Vec::new();
	for i in 0..(width * height) {
		grid.push(0);
	}
	
	let mut tractor_count = 0;
	for y in 0..height {
		for x in 0..width {
			// copy the program because it only runs once.
			let mut prog2 =  super::utility::IntcodeProgram{mem:Vec::new(), pos:prog.pos, relative_base:prog.relative_base};
			for i in 0..prog.mem.len() {
				prog2.mem.push(prog.mem[i]);
			}
			in_buffer.buff.clear();
			in_buffer.buff.push(x);
			in_buffer.buff.push(y);
			in_buffer.read_pos = 0;
			in_buffer.write_pos = 2;
			out_buffer.buff.clear();
			out_buffer.read_pos = 0;
			out_buffer.write_pos = 0;
			super::utility::intcode_execute(&mut prog2, &mut in_buffer, &mut out_buffer, &mut exit);
			if out_buffer.buff[0] == 1 {
				grid[(y * width + x) as usize] = 1;
				tractor_count+=1;
			}
		}
	}
	
	println!("Result A: {}", tractor_count);
	
	// locate a y such that the furthest right tractor coordinate is 100 greater than the furthest left tractor coordinate 100 lines down.
	// binary search to reduce the search space, then finish with linear search
	let santa_width = 100;
	let mut y_high = 20000;
	let mut y_low = 0;
	while y_high - y_low > 2 {
		let y = (y_high + y_low) / 2;
		let mut x = 0;
		let mut first_x = 0;
		let mut last_x = 0;
		let mut first_x2 = 0;
		let mut last_x2 = 0;
		while get_coord(prog, x, y) == 0 {
			x += 1;
		}
		first_x = x;
		while get_coord(prog, x, y) == 1 {
			x += 1;
		}
		last_x = x - 1;
		x = 0;
		while get_coord(prog, x, y+santa_width-1) == 0 {
			x+=1;
		}
		first_x2 = x;
		while get_coord(prog, x, y+santa_width-1) == 1 {
			x += 1;
		}
		last_x2 = x - 1;
		if last_x - first_x + 1 < santa_width || first_x2 > last_x - santa_width + 1 {
			y_low = y;
			continue;
		}
		else if first_x2 < last_x - santa_width + 1 {
			y_high = y;
			continue;
		}
		else {
			break;
		}
	}
	for y in y_low..y_high+1 {
		let mut x = 0;
		let mut first_x = 0;
		let mut last_x = 0;
		let mut first_x2 = 0;
		let mut last_x2 = 0;
		while get_coord(prog, x, y) == 0 {
			x += 1;
		}
		first_x = x;
		while get_coord(prog, x, y) == 1 {
			x += 1;
		}
		last_x = x - 1;
		x = 0;
		while get_coord(prog, x, y+santa_width-1) == 0 {
			x+=1;
		}
		first_x2 = x;
		while get_coord(prog, x, y+santa_width-1) == 1 {
			x += 1;
		}
		last_x2 = x - 1;
		if last_x - first_x2 == santa_width - 1 {
			println!("Result B: {}", first_x2 * 10000 + y);
			break;
		}
	}
	
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
	
	
	run_probe(&mut prog_a, true);
	
	

}