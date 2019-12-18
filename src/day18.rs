// day 18
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct Coord {
	x:usize,
	y:usize,
}
#[derive(Copy, Clone)]
struct MazeNode {
	visited:bool,
	parent_coord:Coord,
	obstacle:bool,
}
#[derive(Copy, Clone)]
struct Door {
	coord:Coord,
	id:u8
}
#[derive(Copy, Clone)]
struct Key {
	coord:Coord,
	id:u8
}

struct MazeGrid {
	grid:Vec<MazeNode>,
	width:usize,
	height:usize,
	origin:Coord,
	doors:Vec<Door>,
	keys:Vec<Key>,
}

// dest must have all vectors initialized
fn copy_mazegrid(src:&mut MazeGrid, dest:&mut MazeGrid) {
	(*dest).grid.clear();
	for i in 0..(*src).grid.len() {
		(*dest).grid.push((*src).grid[i]);
	}
	(*dest).width = (*src).width;
	(*dest).height = (*src).height;
	(*dest).origin = (*src).origin;
	(*dest).doors.clear();
	for i in 0..(*src).doors.len() {
		(*dest).doors.push((*src).doors[i]);
	}
	for i in 0..(*src).keys.len() {
		(*dest).keys.push((*src).keys[i]);
	}
}

struct ElementPosition {
	id:u8,
	dist: usize,
	blocked_by:Vec<u8>,
	steps:usize,
	required_by:usize,
}

fn gridindex(x:usize, y:usize, width:usize)->usize {
	return ((y*(width)) + (x)) as usize;
}

// start must be an explored position, end does not need to be as long as it has an explored neighbor
fn trace_path(grid:&mut MazeGrid, start:Coord, end:Coord)->Vec<Coord> {
	
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
	if !(*grid).grid[gridindex(end.x, end.y, (*grid).width)].visited {
		end_explored = false;
		if (*grid).grid[gridindex(end.x+1, end.y, (*grid).width)].visited && !(*grid).grid[gridindex(end.x+1, end.y, (*grid).width)].obstacle {
			end2 = Coord{x:end.x+1, y:end.y};
		}
		else if (*grid).grid[gridindex(end.x, end.y+1, (*grid).width)].visited && !(*grid).grid[gridindex(end.x, end.y+1, (*grid).width)].obstacle {
			end2 = Coord{x:end.x, y:end.y+1};
		}
		else if (*grid).grid[gridindex(end.x-1, end.y, (*grid).width)].visited && !(*grid).grid[gridindex(end.x-1, end.y, (*grid).width)].obstacle {
			end2 = Coord{x:end.x-1, y:end.y};
		}
		else if (*grid).grid[gridindex(end.x, end.y-1, (*grid).width)].visited && !(*grid).grid[gridindex(end.x, end.y-1, (*grid).width)].obstacle {
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
	
	//println!("\tend2 {} {}", end2.x, end2.y);
	
	let mut start_nodes:Vec<Coord> = Vec::new();
	let mut end2_nodes:Vec<Coord> = Vec::new();
	
	let mut start_parent:Coord = Coord{x:start.x, y:start.y};
	let mut end2_parent:Coord = Coord{x:end2.x, y:end2.y};
	start_nodes.push(start_parent);
	end2_nodes.push(end2_parent);
	
	loop {
		start_parent = (*grid).grid[gridindex(start_parent.x, start_parent.y, (*grid).width)].parent_coord;
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
		if start_parent.x == (*grid).origin.x &&  start_parent.y == (*grid).origin.y {
			break;
		}
	}
	
	loop {
		end2_parent = (*grid).grid[gridindex(end2_parent.x, end2_parent.y, (*grid).width)].parent_coord;
		for i in 1..start_nodes.len() {
			if end2_parent.x == start_nodes[i].x && end2_parent.y == start_nodes[i].y {
				for j in 1..i {
					path.push(start_nodes[j]);
				}
				let mut j:i64 = (end2_nodes.len() as i64) - 1 ;
				while j >= 1 {
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
		if end2_parent.x == (*grid).origin.x &&  end2_parent.y == (*grid).origin.y{
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

fn doors_in_path(grid:&mut MazeGrid, path:&Vec<Coord>)->Vec<u8> {
	let mut door_ids:Vec<u8> = Vec::new();
	if path.len() == 0 {
		return door_ids;
	}
	for i in 0..path.len() - 1 { // ignore final coordinate
		for j in 0..(*grid).doors.len() {
			if path[i].x == (*grid).doors[j].coord.x && path[i].y == (*grid).doors[j].coord.y {
				door_ids.push((*grid).doors[j].id);
			}
		}
	}
	return door_ids;
}

fn path_includes_door(grid:&mut MazeGrid, path:&Vec<Coord>, verbose:bool)->bool {
	if path.len() == 0 {
		return false;
	}
	for i in 0..path.len() - 1 { // ignore final coordinate
		for j in 0..(*grid).doors.len() {
			if path[i].x == (*grid).doors[j].coord.x && path[i].y == (*grid).doors[j].coord.y {
				if verbose {
					println!("door {} found at step {}", ((*grid).doors[j].id + 65) as char, i);
				}
				return true;
			}
		}
	}
	return false;
}

fn key_steps_recurse(keys:&Vec<ElementPosition>, id:u8)->usize {
	for i in 0..keys.len() {
		if keys[i].id == id {
			if keys[i].blocked_by.len() == 0 {
				return 1 as usize;
			}
			else {
				let mut steps:usize = 0;
				for j in 0..keys[i].blocked_by.len() {
					steps += key_steps_recurse(keys, keys[i].blocked_by[j]);
				}
				return 1+steps;
			}
		}
	}
	return 0;
}

fn get_keys_recurse(grid: &mut MazeGrid, steps:usize, position:Coord)->usize {
	return 0;
}

fn get_keys(grid:&mut MazeGrid)->usize {
	let mut position = (*grid).origin;
	let mut steps:usize = 0;
	
	loop {
		if (*grid).keys.len() == 0 {
			break;
		}
	
		// determine accessibility of keys from current position
		let mut keys:Vec<ElementPosition> = Vec::new();
		for i in 0..(*grid).keys.len() {
			let trace = trace_path(grid, position, (*grid).keys[i].coord);
			keys.push(ElementPosition{id:(*grid).keys[i].id, dist:trace.len(), blocked_by:doors_in_path(grid, &trace), steps:0, required_by:0});
		}
		for i in 0..keys.len() {
			keys[i].steps = key_steps_recurse(&keys, keys[i].id);
			for j in 0..keys[i].blocked_by.len() {
				for k in 0..keys.len() {
					if keys[k].id == keys[i].blocked_by[j] {
						keys[k].required_by += 1;
					}
				}
			}
		}
		
		keys.sort_by(cmp_element_position);
		println!("Keys");
		for i in 0..keys.len() {
			println!("\t{} {} {} steps {} required by {}", i, (keys[i].id + 97) as char, keys[i].dist, keys[i].steps, keys[i].required_by);
			for j in 0..keys[i].blocked_by.len() {
				println!("\t\t{}", (keys[i].blocked_by[j] + 65) as char);
			}
		}
		
		if keys[0].steps > 1 {
			println!("No accessible keys available");
			return std::usize::MAX;
		}
		
		let key_id = keys[0].id;
		steps += keys[0].dist;
		for i in 0..(*grid).keys.len() {
			if (*grid).keys[i].id == key_id {
				position = (*grid).keys[i].coord;
				(*grid).keys.remove(i);
				break;
			}
		}
		for i in 0..(*grid).doors.len() {
			if(*grid).doors[i].id == key_id {
				(*grid).doors.remove(i);
				break;
			}
		}
		println!("Key {} {}", (key_id + 65) as char, steps);
	}
	
	println!("Final dist {}", steps);
	return steps;
}

fn explore_grid(grid:&mut MazeGrid) {
	let position:Coord = Coord{x:(*grid).origin.x,y:(*grid).origin.y};
	(*grid).grid[gridindex((*grid).origin.x, (*grid).origin.y, (*grid).width)].visited = true;
	(*grid).grid[gridindex((*grid).origin.x, (*grid).origin.y, (*grid).width)].parent_coord = (*grid).origin;
	let mut possibles:Vec<Coord> = Vec::new();
	
	if !(*grid).grid[gridindex(position.x + 1, position.y, (*grid).width)].visited && !(*grid).grid[gridindex(position.x + 1, position.y, (*grid).width)].obstacle {
			possibles.push(Coord{x:position.x+1,y:position.y});
	}
	if !(*grid).grid[gridindex(position.x - 1, position.y, (*grid).width)].visited && !(*grid).grid[gridindex(position.x - 1, position.y, (*grid).width)].obstacle {
			possibles.push(Coord{x:position.x-1,y:position.y});
	}
	if !(*grid).grid[gridindex(position.x, position.y + 1, (*grid).width)].visited && !(*grid).grid[gridindex(position.x, position.y + 1, (*grid).width)].obstacle {
			possibles.push(Coord{x:position.x,y:position.y + 1});
	}
	if !(*grid).grid[gridindex(position.x, position.y - 1, (*grid).width)].visited && !(*grid).grid[gridindex(position.x, position.y - 1, (*grid).width)].obstacle {
			possibles.push(Coord{x:position.x,y:position.y - 1});
	}
	
	loop {
		//println!("Possibles {}", possibles.len());
		if possibles.len() == 0 {
			break;
		}
		let mut nearest_index = 0;
		let mut min_dist:usize = (*grid).width * (*grid).height;
		for i in 0..possibles.len() {
			let trace = trace_path(grid, position, possibles[i]);
			if trace.len() < min_dist {
				min_dist = trace.len();
				nearest_index = i;
			}
		}
		let path = trace_path(grid, position, possibles[nearest_index]);
		
		(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y, (*grid).width)].visited = true;
		if path.len() > 1 {
			(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y, (*grid).width)].parent_coord = path[path.len() - 2];
		}
		else {
			(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y, (*grid).width)].parent_coord = position;
		}
		
		// add any new neighbors
		if !(*grid).grid[gridindex(possibles[nearest_index].x + 1, possibles[nearest_index].y, (*grid).width)].visited 
			&& !(*grid).grid[gridindex(possibles[nearest_index].x + 1, possibles[nearest_index].y, (*grid).width)].obstacle {
			let temp = Coord{x:possibles[nearest_index].x + 1, y:possibles[nearest_index].y};
			add_once(&mut possibles, temp);
		}
		if !(*grid).grid[gridindex(possibles[nearest_index].x - 1, possibles[nearest_index].y, (*grid).width)].visited 
			&& !(*grid).grid[gridindex(possibles[nearest_index].x - 1, possibles[nearest_index].y, (*grid).width)].obstacle {
			let temp = Coord{x:possibles[nearest_index].x - 1, y:possibles[nearest_index].y};
			add_once(&mut possibles, temp);
		}
		if !(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y + 1, (*grid).width)].visited 
			&& !(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y + 1, (*grid).width)].obstacle {
			let temp = Coord{x:possibles[nearest_index].x, y:possibles[nearest_index].y + 1};
			add_once(&mut possibles, temp);
		}
		if !(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y - 1, (*grid).width)].visited 
			&& !(*grid).grid[gridindex(possibles[nearest_index].x, possibles[nearest_index].y - 1, (*grid).width)].obstacle {
			let temp = Coord{x:possibles[nearest_index].x, y:possibles[nearest_index].y - 1};
			add_once(&mut possibles, temp);
		}
		// remove this from the list of possibles
		possibles.remove(nearest_index);
		
		// for i in 0..possibles.len() {
			// println!("Possible {} {} {}", i, possibles[i].x, possibles[i].y );
		// }
	}
}

fn cmp_door_id(a: &Door, b: &Door) -> Ordering {
    if a.id < b.id {
        return Ordering::Less;
    } else if a.id > b.id {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn cmp_key_id(a: &Key, b: &Key) -> Ordering {
    if a.id < b.id {
        return Ordering::Less;
    } else if a.id > b.id {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

// order by steps, required_by, closest_open_key and dist
fn cmp_element_position(a:&ElementPosition, b:&ElementPosition)->Ordering {
	if a.steps < b.steps  {
		return Ordering::Less;
	}
	else if a.steps > b.steps {
        return Ordering::Greater;
    }
	else {
		if a.required_by < b.required_by {
			return Ordering::Greater;
		}
		else if a.required_by > b.required_by {
			return Ordering::Less;
		}
		// else {
			
			// if a.dist < b.dist {
				// return Ordering::Less;
			// }
			// else if a.dist > b.dist {
				// return Ordering::Greater;
			// }
			
		// }
	}
    return Ordering::Equal;
}

fn read_grid(input:Vec<String>, grid:&mut MazeGrid) {
	(*grid).width = input[0].len();
	(*grid).height = input.len();
	for y in 0..(*grid).height {
		for x in 0..(*grid).width {
			let obstacle;
			let byte = input[y].as_bytes()[x];
			if byte == 35 {
				obstacle = true;
			}
			else {
				obstacle = false;
			}
			// we don't need to map this out if it's an obstacle, so set visited to true
			(*grid).grid.push(MazeNode{visited:obstacle, parent_coord:Coord{x:0,y:0},obstacle:obstacle});
			if !obstacle && byte >= 65 && byte <= 90  {
				(*grid).doors.push(Door{coord:Coord{x:x, y:y}, id:byte - 65});
			}
			else if !obstacle && byte >= 97 && byte <= 122 {
				(*grid).keys.push(Key{coord:Coord{x:x, y:y}, id:byte - 97});
			}
			else if !obstacle && byte == 64 {
				(*grid).origin.x = x;
				(*grid).origin.y = y;
			}
			
			print!("{}", if obstacle {"#"} else {"."});
		}
		println!();
	}
	
	(*grid).doors.sort_by(cmp_door_id);
	(*grid).keys.sort_by(cmp_key_id);
	
	for i in 0..(*grid).keys.len() {
		println!("Key {} {} {} {}", i, (*grid).keys[i].coord.x, (*grid).keys[i].coord.y, ((*grid).keys[i].id + 97) as char);
	}
	for i in 0..(*grid).doors.len() {
		println!("Door {} {} {} {}", i, (*grid).doors[i].coord.x, (*grid).doors[i].coord.y, ((*grid).doors[i].id + 65) as char);
	}
	println!("Origin {} {}", (*grid).origin.x, (*grid).origin.y);
}

pub fn run(file_path:&str) {
	let mut grid = MazeGrid{grid:Vec::new(), origin:Coord{x:0,y:0}, width:0, height:0, doors:Vec::new(), keys:Vec::new()};
	let vec = super::utility::util_fread(file_path);
	read_grid(vec, &mut grid);
	explore_grid(&mut grid);
	get_keys(&mut grid);
}