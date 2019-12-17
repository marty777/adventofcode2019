// day 9 


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
	
	let mut in_buffer_a:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer_a:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	in_buffer_a.buff.push(1);
	in_buffer_a.write_pos = 1;
	
	let mut in_buffer_b:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer_b:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	in_buffer_b.buff.push(2);
	in_buffer_b.write_pos = 1;

	let mut exit:bool = false;
	
	super::utility::intcode_execute(&mut prog_a, &mut in_buffer_a, &mut out_buffer_a, &mut exit);
	println!("Result A: {}", if out_buffer_a.buff.len() == 1 {out_buffer_a.buff[0]} else {0});
	
	exit = false;
	super::utility::intcode_execute(&mut prog_b, &mut in_buffer_b, &mut out_buffer_b, &mut exit);
	println!("Result B: {}", if out_buffer_b.buff.len() == 1 {out_buffer_b.buff[0]} else {0});
	
	
}