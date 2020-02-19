// day 25
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
	North,
	East,
	South,
	West,
}

struct TextNode {
	north: i64,
	east: i64,
	west: i64,
	south: i64,
	name: String,
	explored:bool,
	dest:bool,
}

#[derive(Clone)]
struct DNode {
	dist:usize,
	path:Vec<Dir>
}

// finds path from start to end and returns first direction instruction in path
fn dijkstra(nodes: &mut Vec<TextNode>, start:usize, end:usize, error:&mut bool)->Dir {
	*error = false;
	let mut frontier:HashMap<usize, DNode> = HashMap::new();
	let mut frontier_next:HashMap<usize, DNode> = HashMap::new();
	let mut explored:HashMap<usize, DNode> = HashMap::new();
	
	
	frontier_next.insert(start, DNode{dist:0, path:Vec::new()});
	while frontier_next.len() > 0 {
		frontier.clear();
		for key in frontier_next.keys() {
			let node = frontier_next.get(key).unwrap();
			let node2 = (*node).clone();
			frontier.insert(*key, node2);
		}
		frontier_next.clear();
		for key in frontier.keys() {
			let node = frontier.get(key).unwrap();
			if *key == end {
				if (*node).path.len() > 0 {
					let path_index = 0;
					return (*node).path[path_index];
				}
				else {
					*error = true;
					return Dir::North;
				}
			}
			
			// add to explored or update
			if let Some(explored_node) = explored.get_mut(key) {
				if (*explored_node).dist > (*node).dist {
					(*explored_node).path.clear();
					for i in 0..(*node).path.len() {
						(*explored_node).path.push((*node).path[i]);
					}
					(*explored_node).dist = (*node).dist;
				}
			}
			else {
				let new_node = (*node).clone();
				explored.insert(*key, new_node);
			}
			
			// add next steps to frontier 
			if (*nodes)[*key].north >= 0 {
				let next_index = (*nodes)[*key].north as usize;
				if !explored.contains_key(&next_index) {
					let mut path:Vec<Dir> = Vec::new();
					let dist = (*node).dist + 1;
					for i in 0..(*node).path.len() {
						path.push((*node).path[i]);
					}
					path.push(Dir::North);
					frontier_next.insert(next_index, DNode{dist: dist, path:path});
				}
			}
			
			if (*nodes)[*key].east >= 0 {
				let next_index = (*nodes)[*key].east as usize;
				if !explored.contains_key(&next_index) {
					let mut path:Vec<Dir> = Vec::new();
					let dist = (*node).dist + 1;
					for i in 0..(*node).path.len() {
						path.push((*node).path[i]);
					}
					path.push(Dir::East);
					frontier_next.insert(next_index, DNode{dist: dist, path:path});
				}
			}
			
			if (*nodes)[*key].south >= 0 {
				let next_index = (*nodes)[*key].south as usize;
				if !explored.contains_key(&next_index) {
					let mut path:Vec<Dir> = Vec::new();
					let dist = (*node).dist + 1;
					for i in 0..(*node).path.len() {
						path.push((*node).path[i]);
					}
					path.push(Dir::South);
					frontier_next.insert(next_index, DNode{dist: dist, path:path});
				}
			}
			
			if (*nodes)[*key].west >= 0 {
				let next_index = (*nodes)[*key].west as usize;
				if !explored.contains_key(&next_index) {
					let mut path:Vec<Dir> = Vec::new();
					let dist = (*node).dist + 1;
					for i in 0..(*node).path.len() {
						path.push((*node).path[i]);
					}
					path.push(Dir::West);
					frontier_next.insert(next_index, DNode{dist: dist, path:path});
				}
			}
			
		}
	}
	
	return Dir::North;
}

fn read_node(nodes:&mut Vec<TextNode>, input:String, index:usize, items: &mut Vec<String>) {
	let lines:Vec<&str> = input.as_str().split(10 as char).collect();
	let mut mode = 0;
	if nodes[index].explored {
		return;
	}
	for i in 0..lines.len() {
		let bytes = lines[i].as_bytes();
		if bytes.len() < 2 {
			continue;
		}
		if mode == 0 && bytes[0] == '=' as u8 && bytes[1] == '=' as u8{ // looking for node name
			let mut str_index:usize = 2;
			for _j in 0..bytes.len() {
				if bytes[str_index] != '=' as u8 {
					str_index += 1;
				}
				else {
					break;
				}
			}
			let name = String::from(&lines[i][3..(str_index-1)]);
			(*nodes)[index].name = name;
			mode += 1;
		}
		else if mode == 1 { // looking for door directions start
			if lines[i] == "Doors here lead:" {
				mode += 1;
			}
		}
		else if mode == 2 { // get door directions
			if lines[i] == "Items here:" {
				mode += 1;
				continue;
			}
			// if neighbor doesn't exist, create it and link the new node and the current one
			let mut dest = false;
			if (*nodes)[index].name == "Security Checkpoint" { // mark room branching off security checkpoint as not to be explored
				dest = true;
			}
			
			if lines[i] == "- north" {
				if (*nodes)[index].north == -1 {
					(*nodes).push(TextNode{north:-1, east:-1, south:index as i64, west:-1, name:String::from("unknown"), explored:false, dest:dest});
					(*nodes)[index].north = ((*nodes).len() - 1) as i64;
				}
			}
			else if lines[i] == "- east" {
				if (*nodes)[index].east == -1 {
					(*nodes).push(TextNode{north:-1, east:-1, south:-1, west:index as i64, name:String::from("unknown"), explored:false, dest:dest});
					(*nodes)[index].east = ((*nodes).len() - 1) as i64;
				}
			}
			else if lines[i] == "- south" {
				if (*nodes)[index].south == -1 {
					(*nodes).push(TextNode{north:index as i64, east:-1, south:-1, west:-1, name:String::from("unknown"), explored:false, dest:dest});
					(*nodes)[index].south = ((*nodes).len() - 1) as i64;
				}
			}
			else if lines[i] == "- west" {
				if (*nodes)[index].west == -1 {
					(*nodes).push(TextNode{north:-1, east:index as i64, south:-1, west:-1, name:String::from("unknown"), explored:false, dest:dest});
					(*nodes)[index].west = ((*nodes).len() - 1) as i64;
				}
			}
		}
		else if mode == 3 { // get items
			if &lines[i][0..2] != "- " {
				mode += 1;
			}
			else {
				(*items).push(String::from(&lines[i][2..]));
			}
		}
		else {
			continue;
		}
	}
	
	(*nodes)[index].explored = true;
	
}

fn run_command(prog:&mut super::utility::IntcodeProgram, command:&String, exit:&mut bool, verbose:bool)->String {
	let mut in_buffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut out_buffer = super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0};
	let mut exit1:bool = false;
	if command.len() > 0 {
		let bytes = (*command).as_bytes();
		in_buffer.buff.clear();
		in_buffer.read_pos = 0;
		in_buffer.write_pos = 0;
		for i in 0..bytes.len() {
			if bytes[i] == 13 { // omit carriage return
				continue;
			}
			in_buffer.buff.push(bytes[i] as i64);
		}
		if bytes[bytes.len() - 1] != 10 {
			in_buffer.buff.push(10); // newline
		}
		in_buffer.write_pos = in_buffer.buff.len() - 1;
	}
	
	super::utility::intcode_execute(prog, &mut in_buffer, &mut out_buffer, &mut exit1);
	(*exit) = exit1;
	let mut ret:String = String::from("");
	for i in out_buffer.read_pos..out_buffer.write_pos {
		ret.push(out_buffer.buff[i] as u8 as char);
	}
	
	if verbose {
		println!("##### RUN COMMAND #####");
		println!("INPUT: {}", *command);
		println!("OUTPUT:\n{}\n###############", ret);
	}
	
	return ret;
}

fn run_text_adventure(prog:&mut super::utility::IntcodeProgram) {
	
	let mut nodes:Vec<TextNode> = Vec::new();
	let mut exit:bool = false;
	
	let mut dangerous_goods:Vec<String> = Vec::new();
	dangerous_goods.push(String::from("escape pod"));
	dangerous_goods.push(String::from("giant electromagnet"));
	dangerous_goods.push(String::from("infinite loop"));
	dangerous_goods.push(String::from("photons"));
	dangerous_goods.push(String::from("molten lava"));
	
	let mut inventory:Vec<String> = Vec::new();
	
	let blank_string = String::from("");
	let result = run_command(prog, &blank_string, &mut exit, false);
	nodes.push(TextNode{north:-1, east:-1, south:-1, west:-1, name:String::from("unexplored"), explored:false, dest:false});
	let mut items:Vec<String> = Vec::new();
	read_node(&mut nodes, result, 0, &mut items);
	let mut curr_index = 0;
	
	let mut unexplored = 0; 
	for i in 0..nodes.len() {
		if !nodes[i].explored && !nodes[i].dest {
			unexplored += 1;
		}
	}
	
	// auto-explore and pick up non-dangerous items. Don't try to explore unexplored node off of security checkpoint
	while unexplored > 0 {
		let mut dest_index = 0;
		for i in 0..nodes.len() {
			if !nodes[i].explored && !nodes[i].dest {
				dest_index = i;
				break;
			}
		}
		
		// navigate to dest_index
		let mut error:bool = false;
		let next_dir = dijkstra(&mut nodes, curr_index, dest_index, &mut error);
		if error {
			println!("Pathfinding error between indexes {} and {}", curr_index, dest_index);
			break;
		}
		// issue move command
		let move_command:String;
		let next_index:usize;
		match next_dir {
			Dir::North=>{move_command = String::from("north"); next_index = nodes[curr_index].north as usize},
			Dir::East=>{move_command = String::from("east"); next_index = nodes[curr_index].east as usize},
			Dir::South=>{move_command = String::from("south"); next_index = nodes[curr_index].south as usize},
			_=>{move_command = String::from("west"); next_index = nodes[curr_index].west as usize},
		}
		let result1 = run_command(prog, &move_command, &mut exit, false);
		if exit {
			println!("Program exited with output:\n{}\non command: {}", result1, move_command);
			break;
		}
		let mut items1:Vec<String> = Vec::new();
		read_node(&mut nodes, result1, next_index, &mut items1);
		curr_index = next_index;
		
		for i in 0..items1.len() {
			let mut dangerous:bool = false;
			for j in 0..dangerous_goods.len() {
				if items1[i] == dangerous_goods[j] {
					dangerous = true;
					break;
				}
			}
			if !dangerous {
				let mut take_command = String::from("take ");
				take_command.push_str(items1[i].as_str());
				let result2 = run_command(prog, &take_command, &mut exit, false);
				if exit {
					println!("Program exited with output:\n{}\non command: {}", result2, take_command);
					return;
				}
				let new_string = String::from(items1[i].as_str());
				inventory.push(new_string);
			}
		}

		unexplored = 0;
		for i in 0..nodes.len() {
			if !nodes[i].explored && !nodes[i].dest {
				unexplored += 1;
			}
		}
	}
	
	// all nodes except for the pressure plate are explored. Move to the security checkpoint.
	let mut dest_index:usize = 0;
	let mut security_index:usize = 0;
	for i in 0..nodes.len() {
		if nodes[i].dest {
			dest_index = i;
		}
		else if nodes[i].name == "Security Checkpoint"{
			security_index = i;
		}
	}
	
	while curr_index != security_index {
		let mut error:bool = false;
		let next_dir = dijkstra(&mut nodes, curr_index, security_index, &mut error);
		if error {
			println!("Pathfinding error between indexes {} and {}", curr_index, dest_index);
			return;
		}
		let move_command:String;
		let next_index:usize;
		match next_dir {
			Dir::North=>{move_command = String::from("north"); next_index = nodes[curr_index].north as usize},
			Dir::East=>{move_command = String::from("east"); next_index = nodes[curr_index].east as usize},
			Dir::South=>{move_command = String::from("south"); next_index = nodes[curr_index].south as usize},
			_=>{move_command = String::from("west"); next_index = nodes[curr_index].west as usize},
		}
		let result1 = run_command(prog, &move_command, &mut exit, false);
		if exit {
			println!("Program exited with output:\n{}\non command: {}", result1, move_command);
			return;
		}
		curr_index = next_index;
	}
	
	let end_dir_command:String;
	if nodes[security_index].north == dest_index as i64 {
		end_dir_command = String::from("north");
	}
	else if nodes[security_index].east == dest_index as i64 {
		end_dir_command = String::from("east");
	}
	else if nodes[security_index].south == dest_index as i64 {
		end_dir_command = String::from("south");
	}
	else {
		end_dir_command = String::from("west");
	}
	
	// brute force the puzzle
	let mut state:Vec<bool> = Vec::new();
	for _i in 0..inventory.len() {
		state.push(true);
	}
	
	for i in 1..256 {
		for j in 0..state.len() {
			let mut have = false;
			if i >> j & 0x01 == 1 {
				have = true;
			}
			if have && !state[j] { // drop the item
				let mut command = String::from("take ");
				command.insert_str(5, inventory[j].as_str());
				run_command(prog, &command, &mut exit, false);
			}
			else if !have && state[j] { // pick up the item
				let mut command = String::from("drop ");
				command.insert_str(5, inventory[j].as_str());
				run_command(prog, &command, &mut exit, false);
			}
			state[j] = have;
		}
		
		let result = run_command(prog, &end_dir_command, &mut exit, false);
		if result.contains("heavier") || result.contains("lighter") {
			continue;
		}
		else {
			// get the digit string in the result
			let mut digits:Vec<u8> = Vec::new();
			let bytes = result.as_bytes();
			for i in 0..bytes.len() {
				if bytes[i] >= '0' as u8 && bytes[i] <= '9' as u8 {
					digits.push(bytes[i]);
				}
			}
			print!("Result: ");
			for i in 0..digits.len() {
				print!("{}", digits[i] as char);
			}
			println!();
			break;
		}
	}
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let intcodes_str:Vec<&str> = vec[0].split(",").collect(); 
	let mut prog:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	prog.mem.reserve(intcodes_str.len());
	for code in intcodes_str {
		let temp: i64 = code.parse::<i64>().unwrap();
		prog.mem.push(temp);
	}
	run_text_adventure(&mut prog);
}