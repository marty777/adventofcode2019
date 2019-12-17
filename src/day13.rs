// day 13 


fn run_game(prog:&mut super::utility::IntcodeProgram, part_b:bool)->u64 {
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};

	if !part_b {
		while !exit {
			out_buffer.buff.clear();
			out_buffer.write_pos = 0;
			super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
			
		}
		let mut i = 0;

		let mut block_count:u64 = 0;
		while i < out_buffer.buff.len() {
			if out_buffer.buff[i+2] == 2 {
				block_count += 1;
			}
			i += 3;
		}
		return block_count;
	}
	else {
		let width:i64 = 39; // bounds previously checked on my input
		let height:i64 = 22;
		let mut grid:Vec<u8> = vec![0; ((width) * (height)) as usize];
		let mut score:u64 = 0;
		while !exit {
			out_buffer.buff.clear();
			out_buffer.write_pos = 0;
			super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);			
			let mut i:usize;
			i = 0;
			// rasterize output
			while i < out_buffer.buff.len() {
				if out_buffer.buff[i] == -1 && out_buffer.buff[i+1] == 0 {
					score = out_buffer.buff[i+2] as u64;
					i+=3;
					continue;
				}
				let x = out_buffer.buff[i];
				let y = out_buffer.buff[i+1];
				grid[(y * width + x) as usize] = out_buffer.buff[i+2] as u8;
				i+=3;
			}
			// clear output
			out_buffer.buff.clear();
			out_buffer.write_pos = 0;
			// process raster buffer
			let mut paddle_pos = 0;
			let mut ball_pos = 0;
			for y in 0..height {
				for x in 0..width {
					match grid[(y * width + x) as usize] {
						3=>{paddle_pos = x;},// paddle
						4=>{ball_pos = x;},// ball
						_=>{},// something went wrong
					}
				}
			}
			if in_buffer.buff.len() > 0 {
				in_buffer.buff.clear();
				in_buffer.write_pos = 0;
				in_buffer.read_pos = 0;
			}
			// play the game
			let input;
			if paddle_pos < ball_pos {
				input = 1;
			}
			else if paddle_pos > ball_pos {
				input = -1
			}
			else {
				input = 0;
				
			}
			in_buffer.buff.push(input);
			in_buffer.write_pos = 1;
			if exit {
				return score;
			}
		}
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
	
	println!("Result A: {}", run_game(&mut prog_a, false));
	
	prog_b.mem[0] = 2;
	println!("Result B: {}", run_game(&mut prog_b, true));
}