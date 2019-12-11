// day 7 

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

struct Panel {
	x: i64,
	y: i64,
	color: bool,
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

fn run_robot(prog:&mut IntcodeProgram, part_b:bool)->usize {
	let mut x_pos:i64 = 0;
	let mut y_pos:i64 = 0;
	let mut dir: u8 = 0; // 0 - N, 1 - E, 2 - S, 3 - W
	
	let mut panels:Vec<Panel> = Vec::new();
	let mut exit:bool = false;
	let mut in_buffer:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut white_start = false;
	if part_b {
		white_start = true;
	}
	while !exit {
		// determine current panel color
		let mut found:bool = false;
		let mut found_index = 0;
		for i in 0..panels.len() {
			if panels[i].x == x_pos && panels[i].y == y_pos {
				found = true;
				found_index = i;
			}
		}
		let mut input_color = 0;
		if found && panels[found_index].color {
			input_color = 1;
		}
		else if white_start {
			input_color = 1;
			white_start = false;
		}
		in_buffer.buff.clear();
		in_buffer.read_pos = 0;
		in_buffer.buff.push(input_color);
		in_buffer.write_pos = 1;
		
		execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		let color = out_buffer.buff[0];
		let change_dir = out_buffer.buff[1];
		out_buffer.buff.clear();
		out_buffer.write_pos = 0;
		out_buffer.read_pos = 0;
		
		// check if this panel is already in list
		
		if found {
			panels[found_index].color = color==1;
		}
		else {
			panels.push(Panel{x:x_pos, y:y_pos, color: (color == 1)});
		}
		if change_dir == 0 {
			dir = (dir + 3) % 4;
		}
		else {
			dir = (dir + 1) % 4;
		}
		// advance
		match dir {
			0=>{y_pos += 1;},
			1=>{x_pos += 1;},
			2=>{y_pos -= 1;},
			3=>{x_pos -= 1;},
			_=>{},
		}
		//println!("x:{} y:{} dir:{} panels:{}", x_pos, y_pos, dir, panels.len());
	}
	if !part_b {
		return panels.len();
	}
	
	let mut min_x:i64 = 0;
	let mut min_y:i64 = 0;
	let mut max_x:i64 = 0;
	let mut max_y:i64 = 0;
	for i in 0..panels.len() {
		if panels[i].x < min_x {
			min_x = panels[i].x;
		}
		if panels[i].y < min_y {
			min_y = panels[i].y;
		}
		if panels[i].x > max_x {
			max_x = panels[i].x;
		}
		if panels[i].y > max_y {
			max_y = panels[i].y;
		}
	}
	
	let dim_x = max_x - min_x + 1;
	let dim_y = max_y - min_y + 1;
	let mut grid = vec![false; ((dim_x)*(dim_y)) as usize];
	for i in 0..panels.len() {
		
		if panels[i].color {
			grid[(((panels[i].y - min_y) * dim_x) + (panels[i].x - min_x)) as usize] = true;
		}
	}
	
	for y in 0..dim_y {
		
		for x in 0..dim_x {
			print!("{}", if grid[((dim_y - y - 1)*dim_x + x) as usize] {"#"} else {" "});
		}
		println!();
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
	
	println!("Result A: {}", run_robot(&mut prog_a, false));
	
	println!("Result B: \n");
	
	run_robot(&mut prog_b, true);
}