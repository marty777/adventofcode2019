// day 17

// fn to_iobuffer(iobuff:&mut super::utility::IOBuffer, input:&str) {
	// for c in input.chars() {
		// let val = c as u8;
		// println!("{} {}", val, c);
		// (*iobuff).buff.push(val as i64);
		// (*iobuff).write_pos += 1;
	// }
// }

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
		
		println!("out_buffer {}", out_buffer.buff.len());
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
		// let routine = "A\n";
		// let func_A = "L,12,R,9\n";
		// let func_B = "L,12,R,8\n";
		// let func_C = "L,12,R,8\n";
		// let camera = "y\n";
		// to_iobuffer(&mut in_buffer, routine);
		// to_iobuffer(&mut in_buffer, func_A);
		// to_iobuffer(&mut in_buffer, func_B);
		// to_iobuffer(&mut in_buffer, func_C);
		// to_iobuffer(&mut in_buffer, camera);
		// let mut count = 0;
		
		// super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		// for i in 0..out_buffer.buff.len() {
			// print!("{}", (out_buffer.buff[i] as u8) as char);
		// }
		// in_buffer.buff.clear();
		// in_buffer.read_pos = 0;
		// in_buffer.write_pos = 0;
		// out_buffer.buff.clear();
		// out_buffer.read_pos = 0;
		// out_buffer.write_pos = 0;
		// let routine2 = "B\n";
		// let func_A2 = "L,12,R,8\n";
		// let func_B2 = "L,6\n";
		// let func_C2 = "L,12,R,8\n";
		// let camera2 = "y\n";
		// to_iobuffer(&mut in_buffer, routine2);
		// to_iobuffer(&mut in_buffer, func_A2);
		// to_iobuffer(&mut in_buffer, func_B2);
		// to_iobuffer(&mut in_buffer, func_C2);
		// to_iobuffer(&mut in_buffer, camera2);
		// super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		// for i in 0..out_buffer.buff.len() {
			// print!("{}", (out_buffer.buff[i] as u8) as char);
		// }
		
		println!("Result B: ");
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