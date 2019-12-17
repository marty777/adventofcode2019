// day 15
//use std::io::{self, BufRead};

const MIN_X:i64 = -40;
const MIN_Y:i64 = -40;
const MAX_X:i64 = 39;
const MAX_Y:i64 = 39;

#[derive(Copy, Clone)]
struct Coord {
	x:i64,
	y:i64,
}

struct MazeNode {
	visited:bool,
	parent_coord:Coord,
	o2:bool,
	obstacle:bool,
}

fn gridindex(x:i64, y:i64)->usize {
	return (((y - (MIN_Y))*(MAX_X - MIN_X)) + (x - (MIN_X))) as usize;
}


// start must be an explored position, end does not need to be as long as it has an explored neighbor
fn trace_path(grid:&mut Vec<MazeNode>, start:Coord, end:Coord)->Vec<Coord> {
	
	let mut path:Vec<Coord> = Vec::new();
	// check if start and end are the same
	if start.x == end.x && start.y == end.y {
		return path;
	}
	// check if adjacent 
	if 	(start.x == end.x - 1 && start.y == end.y) ||
		(start.x == end.x + 1 && start.y == end.y) ||
		(start.x == end.x && start.y == end.y - 1) ||
		(start.x == end.x && start.y == end.y + 1)
		{
		path.push(end);
		return path;
	}
	
	let end2:Coord;
	let mut end_explored = true;
	if !(*grid)[gridindex(end.x, end.y)].visited {
		end_explored = false;
		if (*grid)[gridindex(end.x+1, end.y)].visited && !(*grid)[gridindex(end.x+1, end.y)].obstacle {
			end2 = Coord{x:end.x+1, y:end.y};
		}
		else if (*grid)[gridindex(end.x, end.y+1)].visited && !(*grid)[gridindex(end.x, end.y+1)].obstacle {
			end2 = Coord{x:end.x, y:end.y+1};
		}
		else if (*grid)[gridindex(end.x-1, end.y)].visited && !(*grid)[gridindex(end.x-1, end.y)].obstacle {
			end2 = Coord{x:end.x-1, y:end.y};
		}
		else if (*grid)[gridindex(end.x, end.y-1)].visited && !(*grid)[gridindex(end.x, end.y-1)].obstacle {
			end2 = Coord{x:end.x, y:end.y-1};
		}
		else {
			println!("error in trace_path");
			return path;
		}
		// if end2 adjacent to start
		if 	(start.x == end.x - 1 && start.y == end.y) ||
		(start.x == end.x + 1 && start.y == end.y) ||
		(start.x == end.x && start.y == end.y - 1) ||
		(start.x == end.x && start.y == end.y + 1)
		{
			path.push(end2);
			path.push(end);
			return path;
		}
	}
	else {
		end2 = Coord{x:end.x, y:end.y};
	}
		
	
	let mut start_nodes:Vec<Coord> = Vec::new();
	let mut end2_nodes:Vec<Coord> = Vec::new();
	
	let mut start_parent:Coord = Coord{x:start.x, y:start.y};
	let mut end2_parent:Coord = Coord{x:end2.x, y:end2.y};
	start_nodes.push(start_parent);
	end2_nodes.push(end2_parent);
	
	loop {
		start_parent = (*grid)[gridindex(start_parent.x, start_parent.y)].parent_coord;
		
		if start_parent.x == end2_parent.x && start_parent.y == end2_parent.y {
			for i in 1..start_nodes.len() {
				path.push(start_nodes[i]);
			}
			path.push(start_parent);
			if !end_explored {
				path.push(end);
			}
			
			return path;
			
		}
		start_nodes.push(start_parent);
		if start_parent.x == 0 && start_parent.y == 0 {
			break;
		}
	}
	
	loop {
		end2_parent = (*grid)[gridindex(end2_parent.x, end2_parent.y)].parent_coord;
		for i in 1..start_nodes.len() {
			if end2_parent.x == start_nodes[i].x && end2_parent.y == start_nodes[i].y {
				for j in 1..i {
					path.push(start_nodes[j]);
				}
				let mut j:i64 = (end2_nodes.len() as i64) - 2 ;
				while j >= 0 {
					path.push(end2_nodes[j as usize]);
					j-=1;
				}
				path.push(end2);
				if !end_explored {
					path.push(end);
				}
				
				
				return path;
			}
		}
		end2_nodes.push(end2_parent);
		if end2_parent.x == 0 && end2_parent.y == 0 {
			break;
		}
	}
	return path;
}

fn add_once(vec:&mut Vec<Coord>, coord:Coord) {
	let mut found = false;
	for i in 0..(*vec).len() {
		if vec[i].x == coord.x && vec[i].y == coord.y {
			found = true;
			break;
		}
	}
	if !found {
		(*vec).push(coord);
	}
}

fn run_bot(prog:&mut super::utility::IntcodeProgram)->i64 {
	let mut exit:bool = false;
	let mut in_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer:super::utility::IOBuffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	
	let mut grid:Vec<MazeNode> = Vec::new();
	for _y in MIN_Y..MAX_Y {
		for _x in MIN_X..MAX_X {
			grid.push(MazeNode{visited:false, parent_coord:Coord{x:0,y:0}, o2:false, obstacle:false});
		}
	}
	
	
	grid[gridindex(0,0)].visited = true;
	let mut position:Coord = Coord{x:0,y:0};
	let mut possibles:Vec<Coord> = Vec::new();
	possibles.push(Coord{x:-1,y:0});
	possibles.push(Coord{x:0,y:-1});
	possibles.push(Coord{x:1,y:0});
	possibles.push(Coord{x:0,y:1});
	
	
	let mut o2_pos = Coord{x:0,y:0};
	let mut o2_dist = 0;
	
	loop {
		
		if possibles.len() == 0 {
			break;
		}
		let mut nearest_index = 0;
		let mut min_dist:usize = ((MAX_X - MIN_X) * (MIN_X - MAX_X)) as usize;
		for i in 0..possibles.len() {
			let trace = trace_path(&mut grid, position, possibles[i]);
			if trace.len() < min_dist {
				min_dist = trace.len();
				nearest_index = i;
			}
		}
		let path = trace_path(&mut grid, position, possibles[nearest_index]);
	
		// load up the buffer 
		in_buffer.buff.clear();
		in_buffer.read_pos = 0;
		in_buffer.write_pos = 0;
		out_buffer.buff.clear();
		out_buffer.read_pos = 0;
		out_buffer.write_pos = 0;
		for i in 0..path.len() {
			if i == 0 {
				if path[i].x == position.x + 1 && path[i].y == position.y {
					in_buffer.buff.push(4);
				}
				else if path[i].x == position.x - 1&& path[i].y == position.y  {
					in_buffer.buff.push(3);
				}
				else if path[i].y == position.y + 1 && path[i].x == position.x  {
					in_buffer.buff.push(1);
				}
				else if path[i].y == position.y - 1 && path[i].x == position.x{
					in_buffer.buff.push(2);
				}
				else {
					println!("Error with path");
					return 0;
				}
				
			}
			else {
				if path[i].x == path[i-1].x + 1 && path[i].y == path[i-1].y {
					in_buffer.buff.push(4);
				}
				else if path[i].x == path[i-1].x - 1 && path[i].y == path[i-1].y {
					in_buffer.buff.push(3);
				}
				else if path[i].y == path[i-1].y + 1 && path[i].x == path[i-1].x {
					in_buffer.buff.push(1);
				}
				else if path[i].y == path[i-1].y - 1 && path[i].x == path[i-1].x {
					in_buffer.buff.push(2);
				}
				else {
					println!("Error with path");
					return 0;
				}
			}
			in_buffer.write_pos += 1;
		}
		while in_buffer.read_pos < in_buffer.buff.len() {
			super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit);
		}
		if out_buffer.buff[out_buffer.write_pos-1] == 0 {
			if path.len() > 1 {
				position = path[path.len() - 2];
			}
			// otherwise, position remains the same
			grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].visited = true;
			grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].obstacle = true;
		}
		else if out_buffer.buff[out_buffer.write_pos-1] == 1 {
			grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].visited = true;
			if path.len() > 1 {
				grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].parent_coord = path[path.len() - 2];
			}
			else {
				grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].parent_coord = position;
			}
			position = path[path.len() - 1];
			
		}
		else if out_buffer.buff[out_buffer.write_pos-1] == 2{
			grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].visited = true;
			grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].o2 = true;
			if path.len() > 1 {
				grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].parent_coord = path[path.len() - 2];
			}
			else {
				grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y)].parent_coord = position;
			}
			position = path[path.len() - 1];
			
			let o2_trace = trace_path(&mut grid, possibles[nearest_index], Coord{x:0,y:0});
			o2_dist = o2_trace.len();
			o2_pos = possibles[nearest_index];
			
			
		}
		else {
			println!("Unexpected response");
			break;
		}
		
		
		
		// remove this from the list of possibles
		possibles.remove(nearest_index);
		// add any new ones
		if !grid[gridindex(position.x+1, position.y)].visited {
			add_once(&mut possibles, Coord{x:position.x + 1, y:position.y});
		}
		if !grid[gridindex(position.x-1, position.y)].visited {
			add_once(&mut possibles, Coord{x:position.x - 1, y:position.y});
		}
		if !grid[gridindex(position.x, position.y+1)].visited {
			add_once(&mut possibles, Coord{x:position.x, y:position.y+1});
		}
		if !grid[gridindex(position.x, position.y-1)].visited {
			add_once(&mut possibles, Coord{x:position.x, y:position.y-1});
		}
	}
	
	let mut min_explored_x = MAX_X;
	let mut min_explored_y = MAX_Y;
	let mut max_explored_x = MIN_X;
	let mut max_explored_y = MIN_Y;
	for y in MIN_Y..MAX_Y {
		for x in MIN_X..MAX_X {
			if grid[gridindex(x,y)].visited {
				if x < min_explored_x {
					min_explored_x = x;
				}
				if x > max_explored_x {
					max_explored_x = x;
				}
				if y < min_explored_y {
					min_explored_y = y;
				}
				if y > max_explored_y {
					max_explored_y = y;
				}
			}
		}
	}
	let mut max_dist = 0;
	for y in min_explored_y..max_explored_y+1 {
		for x in min_explored_x..max_explored_x+1 {
			if x == o2_pos.x && y == o2_pos.y {
				continue;
			}
			if !grid[gridindex(x,y)].visited || grid[gridindex(x,y)].obstacle {
				continue;
			}
			let trace = trace_path(&mut grid, o2_pos, Coord{x:x,y:y});
			if trace.len() > max_dist {
				max_dist = trace.len();
			}
		}
	}
	
	println!("Result A: {}", o2_dist);
	println!("Result B: {}", max_dist+1);
	
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
	
	run_bot(&mut prog_a);
}