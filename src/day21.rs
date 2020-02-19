// day 21

fn run_droid(prog:&mut super::utility::IntcodeProgram, part_b:bool) {
	
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	
	let part_char:char;
	if part_b {
		part_char = 'B';
	}
	else {
		part_char = 'A';
	}
	
	
	let mut springscript:Vec<&str> = Vec::new();
	if !part_b {
		// jump is 4 spaces
		// if D available and A B or C is not -> jump
		// T and J reset to false each cycle
		// if D && (!A || !B || !C) jump
		// => D && !(A && B && C)
		springscript.push("OR A J");	// J = A (since J initializes to false)
		springscript.push("AND B J");
		springscript.push("AND C J");	// J = A && B && C
		springscript.push("NOT J J");	// J = !(A && B && C)
		springscript.push("AND D J");	// J = D && !(A && B && C)
		springscript.push("WALK");
	}
	else {
		// jump is 4 spaces
		// plot two jumps ahead
		// only jump if A or B or C are empty and D is present
		// if H is empty, either E and I must be present or F must be present (we don't know we can jump from F successfully, but it's the only option)
		// .................
		// .................
		// ..@..............
		// #####.#.#..##.###
		//    ABCDEFGHI
		
		// .................
		// .................
		// ....@............
		// #####..####..####
		//      ABCDEFGHI
		// if (D && H) || (D && E && I) || (D && E && F) && (!A || !B || !C) jump
		// => (D && H) || (D && E && I) || (D && E && F) && !(A && B && C)
		// => (D && H) || (D && E && I) || (D && E && F) = D && (H || E && I || E && F)
		// => D && (H || (E && (I || F)) && !(A && B && C)
		springscript.push("OR A J"); 	// J = A (since J initializes to false)
		springscript.push("AND B J");
		springscript.push("AND C J");	// J = A && B && C
		springscript.push("NOT J J"); 	// J = !(A && B && C)
		springscript.push("OR I T");  
		springscript.push("OR F T");  	// T = I || F
		springscript.push("AND E T");	// T = E && (I || F)
		springscript.push("OR H T");	// T = H || (E && I || F)
		springscript.push("AND D J");	// T = D && (H || (E && (I || F))
		springscript.push("AND T J");	// J = D && (H || (E && (I || F)) && !(A && B && C)
		springscript.push("RUN");
	}
	
	in_buffer.buff.clear();
	in_buffer.read_pos = 0;
	in_buffer.write_pos = 0;
	for i in 0..springscript.len() {
		let bytes = springscript[i].as_bytes();
		for j in 0..bytes.len() {
			in_buffer.buff.push(bytes[j] as i64);
		}
		in_buffer.buff.push(10);
	}
	in_buffer.write_pos = in_buffer.buff.len();
	super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
	if out_buffer.buff.len() > 34 {
		// failure - print out as ASCII
		for i in 0..out_buffer.buff.len() {
			print!("{}", out_buffer.buff[i] as u8 as char);
		}
	}
	else {
		println!("Result {}: {}", part_char, out_buffer.buff[out_buffer.buff.len() - 1]);
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
	
	run_droid(&mut prog_a, false);
	run_droid(&mut prog_b, true);
}