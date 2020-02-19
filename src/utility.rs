// utility
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::vec;

pub struct IntcodeProgram {
	pub mem: Vec<i64>,
	pub pos: i64,
	pub relative_base: i64,
}

pub struct IOBuffer {
	pub buff: Vec<i64>,
	pub write_pos: usize,
	pub read_pos: usize,
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

pub fn intcode_execute_once(prog: &mut IntcodeProgram, input:&mut IOBuffer, output:&mut IOBuffer, exit:&mut bool, io_wait:&mut bool, error: &mut bool) {
	let mut err:bool = false;
	
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
			if (*input).read_pos < (*input).buff.len() {
				let input_val = (*input).buff[(*input).read_pos];
				(*input).read_pos += 1;
				set(prog, (*prog).pos + 1, mode1, input_val, &mut err);
				(*prog).pos += 2;
			}
			else {
				// return until input available in buffer
				*exit = false;
				*io_wait = true;
			}
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
		99 => {*exit = true; *io_wait = false;}
		_ => {println!("Invalid opcode found {}", opcode); *error = true;}
	}
}

pub fn intcode_execute(prog: &mut IntcodeProgram, input:&mut IOBuffer, output:&mut IOBuffer, exit:&mut bool) {
	let mut instruction_error = false;
	let mut instruction_exit = false;
	let mut instruction_io_wait = false;
	loop {
		intcode_execute_once(prog, input, output, &mut instruction_exit, &mut instruction_io_wait, &mut instruction_error);
		if instruction_exit {
			*exit = true;
			return;
		}
		else if instruction_io_wait || instruction_error {
			*exit = false;
			return;
		}
	}
}

// read by line, returning a vector of Strings
pub fn util_fread(file_path:&str) -> std::vec::Vec<String>{
	let mut vec:Vec<String> = Vec::new();
	
	let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
		vec.push(line);
    }
	
	return vec;
}