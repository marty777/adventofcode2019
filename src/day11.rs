// day 11 


struct Panel {
	x: i64,
	y: i64,
	color: bool,
}

fn run_robot(prog:&mut super::utility::IntcodeProgram, part_b:bool)->usize {
	let mut x_pos:i64 = 0;
	let mut y_pos:i64 = 0;
	let mut dir: u8 = 0; // 0 - N, 1 - E, 2 - S, 3 - W
	
	let mut panels:Vec<Panel> = Vec::new();
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
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
		
		super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
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
	let mut prog_a:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	let mut prog_b:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
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