// day 13 

struct IntcodeProgram {
	mem: Vec<i64>,
	pos: i64,
	relative_base: i64,
}

struct IOBuffer {
	buff: Vec<i64>,
	write_pos: usize,
	read_pos: usize,
}

fn expand_mem(program: &mut IntcodeProgram, pos:usize) {
	if (*program).mem.len() <= pos {
		(*program).mem.reserve(pos + 1 - (*program).mem.len());
		for _i in (*program).mem.len().. ((pos+1)) {
			(*program).mem.push(0);
		}
	}
}

fn get(program:&mut IntcodeProgram, pos:i64, mode:i64, err:&mut bool) -> i64 {
	// if address outside range, expand memory
	if pos < 0 {
		*err = true;
		return 0;
	}
	expand_mem(program, pos as usize);
	let val = (*program).mem[pos as usize];
	match mode {
		0=>{expand_mem(program, val as usize);return (*program).mem[val as usize];},
		1=>{return val;},
		2=>{expand_mem(program, (val + (*program).relative_base) as usize); return (*program).mem[(val + (*program).relative_base) as usize];},
		_=>{*err = true; return 0;}
	}
}

fn set(program: &mut IntcodeProgram, pos:i64, mode:i64, value: i64, err: &mut bool) {
	if mode == 1 {
		*err = true;
		return;
	}
	if pos < 0 {
		*err = true;
		return;
	}
	expand_mem(program, pos as usize);
	let val = (*program).mem[pos as usize];
	match mode {
		0=>{expand_mem(program, val as usize);(*program).mem[val as usize] = value;},
		2=>{expand_mem(program, (val + (*program).relative_base) as usize); (*program).mem[(val + (*program).relative_base) as usize] = value; },
		_=>{*err = true;}
	}
	return;
}

fn execute(prog: &mut IntcodeProgram, input:&mut IOBuffer, output:&mut IOBuffer, exit:&mut bool) {
	let mut err:bool = false;
	loop  {
		let memval = get(prog, (*prog).pos, 1, &mut err);
		let opcode = memval % 100;
		let mode1 = ((memval - opcode) % 1000)/100;
		let mode2 = ((memval - opcode - 100*mode1) % 10000)/1000;
		let mode3 = ((memval - opcode - 100*mode1 - 1000*mode2) % 100000)/10000; // presumably this will be used in future opcodes. Unused at the moment.
		
		
		match opcode {
			1 => { // add
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				
				let val = param1 + param2;
				set(prog, (*prog).pos + 3, mode3, val, &mut err);
				
				(*prog).pos += 4;
			},
			2 => { // mul
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				
				let val = param1 * param2;
				set(prog, (*prog).pos + 3, mode3, val, &mut err);
				
				(*prog).pos += 4;
			},
			3 => { // input
				//let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				if (*input).read_pos < (*input).buff.len() {
					let input_val = (*input).buff[(*input).read_pos];
					(*input).read_pos += 1;
					set(prog, (*prog).pos + 1, mode1, input_val, &mut err);
				}
				else {
					// return until input available in buffer
					*exit = false;
					break;
				}
				(*prog).pos += 2;
				
			},
			4 => { // output
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				(*output).buff.push(param1);
				(*output).write_pos += 1;
				
				(*prog).pos += 2;
				
			},
			5 => { // jump-if-true
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				if param1 != 0 {
					(*prog).pos = param2;
				}
				else {
					(*prog).pos += 3;
				}
			},
			6 => { // jump-if-true
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				if param1 == 0 {
					(*prog).pos = param2;
				}
				else {
					(*prog).pos += 3;
				}
			},
			7 => { // lt
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				if param1 < param2 {
					set(prog, (*prog).pos + 3, mode3, 1, &mut err);
				}
				else {
					set(prog, (*prog).pos + 3, mode3, 0, &mut err);
				}
				(*prog).pos += 4;
			},
			8 => { // eq
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				let param2 = get(prog, (*prog).pos + 2, mode2, &mut err);
				if param1 == param2 {
					set(prog, (*prog).pos + 3, mode3, 1, &mut err);
				}
				else {
					set(prog, (*prog).pos + 3, mode3, 0, &mut err);
				}
				(*prog).pos += 4;
			},
			9 => { // relative base
				let param1 = get(prog, (*prog).pos + 1, mode1, &mut err);
				(*prog).relative_base += param1;
				(*prog).pos += 2;
				
			},
			99 => {*exit = true; break;}
			_ => {println!("Invalid opcode found {}", opcode); break;}
		}
	}
	return;
}

fn run_game(prog:&mut IntcodeProgram, part_b:bool)->u64 {
	let mut exit:bool = false;
	let mut in_buffer:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};

	if !part_b {
		while !exit {
			out_buffer.buff.clear();
			out_buffer.write_pos = 0;
			execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
			
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
			execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);			
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
	let mut prog_a:IntcodeProgram = IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	let mut prog_b:IntcodeProgram = IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
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