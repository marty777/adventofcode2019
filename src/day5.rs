// day 5 

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
	let mut exit:bool = false;
	let mut in_buffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	
	in_buffer.buff.push(1);
	in_buffer.write_pos = 1;
	super::utility::intcode_execute(&mut prog_a, &mut in_buffer, &mut out_buffer, &mut exit);
	println!("Result A: {}", out_buffer.buff[out_buffer.buff.len() - 1]);
	
	// clear buffers
	in_buffer.buff.clear();
	in_buffer.read_pos = 0;
	in_buffer.write_pos = 0;
	out_buffer.buff.clear();
	out_buffer.read_pos = 0;
	out_buffer.write_pos = 0;
	out_buffer.write_pos = 0;
	
	in_buffer.buff.push(5);
	in_buffer.write_pos = 1;
	
	// run fresh copy of program
	super::utility::intcode_execute(&mut prog_b, &mut in_buffer, &mut out_buffer, &mut exit);
	println!("Result B: {}", out_buffer.buff[out_buffer.buff.len() - 1]);
}