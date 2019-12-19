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
	cached_distances: Vec<usize>,
	key_requirements:Vec<Vec<u8>>,
	keys_in_path:Vec<Vec<u8>>,
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

struct OrderResult {
	result:i64,
	order:Vec<u8>,
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
	
	for i in 0..start_nodes.len() {
		//println!("start node {} {}", start_nodes[i].x, start_nodes[i].y);
	}
	
	loop {
		end2_parent = (*grid).grid[gridindex(end2_parent.x, end2_parent.y, (*grid).width)].parent_coord;
		// if we hit the start coordinates directly 
		if end2_parent.x == start.x && end2_parent.y == start.y {
			//println!("start and end in same branch");
			let mut j:i64 = (end2_nodes.len() as i64) - 1;
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
		for i in 1..start_nodes.len() {
			if end2_parent.x == start_nodes[i].x && end2_parent.y == start_nodes[i].y {
				for j in 1..i {
					path.push(start_nodes[j]);
				}
				if end2_parent.x == (*grid).origin.x && end2_parent.y == (*grid).origin.y && !(start.x == (*grid).origin.x && start.y == (*grid).origin.y) {
					path.push((*grid).origin);
				}
				else {
					path.push(end2_parent);
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
				println!("End 2 {} {} end {} {} start {} {}", end2.x, end2.y, end.x, end.y, start.x, start.y);
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

fn keys_in_path(grid:&mut MazeGrid, path:&Vec<Coord>)->Vec<u8> {
	let mut key_ids:Vec<u8> = Vec::new();
	if path.len() == 0 {
		return key_ids;
	}
	for i in 0..path.len() - 1 { // ignore final coordinate
		for j in 0..(*grid).keys.len() {
			if path[i].x == (*grid).keys[j].coord.x && path[i].y == (*grid).keys[j].coord.y {
				key_ids.push((*grid).keys[j].id);
			}
		}
	}
	return key_ids;
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

fn get_keys_recurse2(grid: &mut MazeGrid, order:Vec<u8>, depth:usize)->OrderResult {
	if(*grid).keys.len() < 1 {
		return OrderResult{result:-1, order:Vec::new()};
	}
	for _i in 0..depth {
		print!("\t");
	}
	print!("recurse depth {}: ", depth);
	for i in 0..order.len() {
		print!("{},", (order[i] + 65) as char);
	}
	// test if this is a valid ordering so far. All required keys to reach a given key must appear previously in order
	let mut valid = true;
	for i in 0..order.len() {
		let key = order[i] as usize;
		for j in 0..(*grid).key_requirements[key].len() {
			let mut found = false;
			for k in 0..i {
				if order[k] == (*grid).key_requirements[key][j] {
					found = true;
					break;
				}
			}
			if !found {
				valid = false;
				break;
			}
		}
		if !valid {
			break;
		}
	}
	
	if !valid {
		println!(" Invalid ");
		return OrderResult{result:-1, order:Vec::new()};
	}
	
	// if we're done, return the path length
	if order.len() == (*grid).keys.len() {
		let mut a = (*grid).keys.len();
		let mut b;
		let mut path_len = 0;
		for i in 0..order.len() {
			b = order[i] as usize;
			path_len += (*grid).cached_distances[(a * ((*grid).keys.len() + 1)) + b];
			a = b;
		}
		println!(" Final {}", path_len);
		let mut result = OrderResult{result:path_len as i64, order:Vec::new()};
		for i in 0..order.len() {
			result.order.push(order[i]);
		}
		return result;
	}
	else {
		// try all next steps 
		let mut min_path = std::i64::MAX;
		let mut min_order:Vec<u8> = Vec::new();
		for i in 0..(*grid).keys.len() {
			let mut already_used = false;
			for j in 0..order.len() {
				if order[j] == i as u8 {
					already_used = true;
					break;
				}
			}
			if already_used {
				continue;
			}
			
			// test if the path to this key from the origin contains any other keys.
			// if they haven't been used, continue
			let mut preceding_keys_not_used = false;
			for j in 0..(*grid).keys_in_path[i].len() {
				let mut found_key = false;
				for k in 0..order.len() {
					if order[k] == (*grid).keys_in_path[i][j] {
						found_key = true;
						break;
					}
				}
				if !found_key {
					println!("Could not find preceding key {} for {}", (i as u8 + 65) as char, ((*grid).keys_in_path[i][j] as u8 + 65) as char);
					preceding_keys_not_used = true;
					break;
				}
			}
			if preceding_keys_not_used {
				println!("skipping {} due to unused preceding keys", (i as u8 + 65) as char);
				continue;
			}
			
			let mut order2:Vec<u8> = Vec::new();
			for j in 0..order.len() {
				order2.push(order[j]);
			}
			order2.push(i as u8);
			println!();
			let test_result = get_keys_recurse2(grid, order2, depth + 1);
			if test_result.result > -1 && test_result.result < min_path {
				min_path = test_result.result;
				min_order.clear();
				for i in 0..test_result.order.len() {
					min_order.push(test_result.order[i]);
				}
			}
		}
		println!();
		let mut result = OrderResult{result:min_path, order:Vec::new()};
		for i in 0..min_order.len() {
			result.order.push(min_order[i]);
		}
		return result;
	}
	
}

fn get_keys_recurse(grid: &mut MazeGrid, steps:usize, position:Coord)->usize {
	if (*grid).keys.len() == 1 {
		let trace = trace_path(grid, position, (*grid).keys[0].coord);
		println!("\t recurse final {} {} {}", ((*grid).keys[0].id + 65) as char,  steps + trace.len(), trace.len());
		for i in 0..trace.len() {
			println!("\t\t{},{}", trace[i].x, trace[i].y);
		}
		return steps + trace.len();
	}
	else {
		// get list of keys
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
		
		keys.sort_by(cmp_element_position2);
		
		println!("\tAvailable recurse keys");
		for i in 0..keys.len() {
			println!("\t\t{}", (keys[i].id + 65 ) as char);
		}
		
		let mut candidates:Vec<u8> = Vec::new();
		candidates.push(keys[0].id);
		for i in 1..keys.len() {
			if cmp_element_position2(&keys[i], &keys[0]) != Ordering::Equal {
				break;
			}
			candidates.push(keys[i].id);
		}
		
		println!("\tRecurse candidates");
		for i in 0..candidates.len() {
			println!("\t\t{}", (candidates[i] + 65) as char);
		}
		
		if candidates.len() > 1 {
			// try each options, return best one
			println!("Testing candidates");
			let mut min_steps = std::usize::MAX;
			let mut min_candidate:u8 = 0;
			for i in 0..candidates.len() {
				// make new copy of grid
				let mut grid2 = MazeGrid{grid:Vec::new(), width:0, height:0, origin:Coord{x:0,y:0}, keys:Vec::new(), doors:Vec::new(), cached_distances:Vec::new(), key_requirements:Vec::new(), keys_in_path:Vec::new()};
				copy_mazegrid(grid, &mut grid2);
				
				// try the candidate
				let mut candidate_index = 0;
				for j in 0..grid2.keys.len() {
					if grid2.keys[j].id == candidates[i] {
						candidate_index = i;
						break;
					}
				}
				let position2 = grid2.keys[candidate_index].coord;
				let trace = trace_path(&mut grid2, position, position2);
				
				for j in 0..grid2.keys.len() {
					if grid2.keys[j].id == candidates[i] {
						grid2.keys.remove(j);
						break;
					}
				}
				for j in 0..grid2.doors.len() {
					if grid2.doors[j].id == candidates[i] {
						grid2.doors.remove(j);
						break;
					}
				}
				let steps2 =  get_keys_recurse(&mut grid2, steps + trace.len(), position2);
				if steps2 < min_steps {
					min_steps = steps2;
					min_candidate = candidates[i];
				}
			}
			println!("Selected candidate {} {}", (min_candidate + 65) as char, min_steps);
			// remove candidates other than the selected one
			candidates.clear();
			candidates.push(min_candidate);
		}
		
		if candidates.len() == 1 {
			let mut candidate_index = 0;
			for i in 0..(*grid).keys.len() {
				if (*grid).keys[i].id == candidates[0] {
					candidate_index = i;
					break;
				}
			}
			let position2 = (*grid).keys[candidate_index].coord;
			let trace = trace_path(grid, position, position2);
			
			for i in 0..(*grid).keys.len() {
				if (*grid).keys[i].id == candidates[0] {
					(*grid).keys.remove(i);
					break;
				}
			}
			for i in 0..(*grid).doors.len() {
				if (*grid).doors[i].id == candidates[0] {
					(*grid).doors.remove(i);
					break;
				}
			}
			println!("Recur {} {} {} {},{}", steps + trace.len(), steps, trace.len(), position2.x, position2.y);
			for i in 0..trace.len() {
				println!("\t{},{}", trace[i].x, trace[i].y);
			}
			return get_keys_recurse(grid, steps + trace.len(), position2);
		}
		else {
			println!("An error occured in recursion");
			return 0;
		}
	}
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
	
	// determine distances between all keys. keys.len+1 row/col is for distances to origin
	for a in 0..(*grid).keys.len() + 1 {
		let mut position_a = Coord{x:0,y:0};
		if a == (*grid).keys.len() {
			position_a = (*grid).origin;
		}
		else {
			for i in 0..(*grid).keys.len() {
				if (*grid).keys[i].id == a as u8 {
					position_a = (*grid).keys[i].coord;
				}
			}
		}
		for b in 0..(*grid).keys.len() + 1 {
			if b == a {
				(*grid).cached_distances.push(0);
				continue;
			}
			let mut position_b = Coord{x:0,y:0};
			if b == (*grid).keys.len() {
				position_b = (*grid).origin;
			}
			else {
				for i in 0..(*grid).keys.len() {
					if (*grid).keys[i].id == b as u8 {
						position_b = (*grid).keys[i].coord;
					}
				}
			}
			let trace = trace_path(grid, position_a, position_b);
			(*grid).cached_distances.push(trace.len());
		}
	}
	
	// cache which keys require other keys to be accessible
	for i in 0..(*grid).keys.len() {
		let trace = trace_path(grid, (*grid).origin, (*grid).keys[i].coord);
		let requirements = doors_in_path(grid, &trace);
		println!("{} {} {}", i, (*grid).keys[i].id, ((i+65) as u8) as char );
		for i in 0..requirements.len() {
			println!("\t{}", (requirements[i] + 65) as char)
		}
		(*grid).key_requirements.push(requirements);
	}
	
	// cache which keys appear in the path to reach a given key
	for i in 0..(*grid).keys.len() {
		let trace = trace_path(grid, (*grid).origin, (*grid).keys[i].coord);
		let keys_in_path = keys_in_path(grid, &trace);
		println!("{} {} {}", i, (*grid).keys[i].id, ((i+65) as u8) as char );
		for i in 0..keys_in_path.len() {
			println!("\t{}", (keys_in_path[i] + 65) as char)
		}
		(*grid).keys_in_path.push(keys_in_path);
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

fn cmp_element_position2(a:&ElementPosition, b:&ElementPosition)->Ordering {
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
		else {
			
			if a.dist < b.dist {
				return Ordering::Less;
			}
			else if a.dist > b.dist {
				return Ordering::Greater;
			}
			
		}
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
	let mut grid = MazeGrid{grid:Vec::new(), origin:Coord{x:0,y:0}, width:0, height:0, doors:Vec::new(), keys:Vec::new(), cached_distances:Vec::new(), key_requirements:Vec::new(), keys_in_path:Vec::new()};
	let vec = super::utility::util_fread(file_path);
	read_grid(vec, &mut grid);
	explore_grid(&mut grid);
	let position = grid.origin;
	//println!("{}", get_keys_recurse(&mut grid, 0, position));
	let order:Vec<u8> = Vec::new();
	let result = get_keys_recurse2(&mut grid, order, 0);
	println!("Result: {}", result.result);
	for i in 0..result.order.len() {
		let mut len = 0;
		if i == 0 {
			len = grid.cached_distances[((grid.keys.len())*(grid.keys.len() + 1) + result.order[i] as usize)];
		}
		else {
			len = grid.cached_distances[((result.order[i-1] as  usize)*(grid.keys.len() + 1) + result.order[i] as usize)];
		}
		println!("{} {} {}", i, (result.order[i] + 65) as char, len);
	}
	// println!("O->A {}", grid.cached_distances[(6 * (grid.keys.len() + 1)) + 0]);
	// println!("A->B {}", grid.cached_distances[(0 * (grid.keys.len() + 1)) + 1]);
	// println!("B->D {}", grid.cached_distances[(1 * (grid.keys.len() + 1)) + 3]);
	// println!("D->C {}", grid.cached_distances[(3 * (grid.keys.len() + 1)) + 2]);
	// println!("C->E {}", grid.cached_distances[(2 * (grid.keys.len() + 1)) + 4]);
	// println!("E->F {}", grid.cached_distances[(4 * (grid.keys.len() + 1)) + 5]);
	
	 let coord1 = grid.keys[5].coord;
	 let coord2 = grid.keys[6].coord;
	 let trace = trace_path(&mut grid, coord1, coord2);
	 for i in 0..trace.len() {
		 println!("{} {},{}", i, trace[i].x, trace[i].y);
	 }
	 
	 for i in 0..grid.keys_in_path.len() {
		println!("Key {} precedes", (i as u8 + 65) as char);
		for j in 0..grid.keys_in_path[i].len() {
			println!("\t {}", (grid.keys_in_path[i][j] + 65) as char);
		}
	 }
}