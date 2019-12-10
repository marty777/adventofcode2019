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
	
	let mut in_buffer_a:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer_a:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	in_buffer_a.buff.push(1);
	in_buffer_a.write_pos = 1;
	
	let mut in_buffer_b:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer_b:IOBuffer = IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	in_buffer_b.buff.push(2);
	in_buffer_b.write_pos = 1;

	let mut exit:bool = false;
	
	execute(&mut prog_a, &mut in_buffer_a, &mut out_buffer_a, &mut exit);
	println!("Result A: {}", if out_buffer_a.buff.len() == 1 {out_buffer_a.buff[0]} else {0});
	
	exit = false;
	execute(&mut prog_b, &mut in_buffer_b, &mut out_buffer_b, &mut exit);
	println!("Result B: {}", if out_buffer_b.buff.len() == 1 {out_buffer_b.buff[0]} else {0});
	
	
}