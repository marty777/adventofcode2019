// day 18
use std::collections::HashMap;

struct MazeNode {
	obstacle: bool,
	key_index: i64,
	door_index: i64,
}

struct Key {
	x: usize,
	y: usize,
	symbol: char,
}

struct Door {
	_x: usize,
	_y: usize,
	symbol: char,
	key_index: usize,
}

struct Maze {
	grid: Vec<Vec<MazeNode>>,
	keys: Vec<Key>,
	doors: Vec<Door>,
	cached: HashMap<usize, HashMap<usize, CachedPath>>,
	width: usize,
	height: usize,
}

struct CachedPath {
	dist: i64,
	keys: Vec<usize>,
}

#[derive(Clone)]
struct DNode {
	x: usize,
	y: usize,
	dist: usize,
	parent_x:usize,
	parent_y:usize
}

#[derive(Clone)]
struct DNodeB {
	at:Vec<usize>,
	keys:Vec<usize>,
	dist:usize
}

fn intersect_count (vec_a:&Vec<usize>, vec_b:&Vec<usize>)->usize {
	let mut count = 0;
	for i in 0..vec_a.len() {
		for j in 0..vec_b.len() {
			if vec_b[j] == vec_a[i] {
				count+=1;
				break;
			}
		}
	}
	return count;
}

fn keynodeindex(maze:&mut Maze, keys: &Vec<usize>, at: &Vec<usize>)->String {
	let mut ret = String::from("");
	let mut keys2 = Vec::new();
	for i in 0..keys.len() {
		keys2.push((*maze).keys[(*keys)[i]].symbol);
	}
	keys2.sort();
	for i in 0..(*at).len() {
		ret.push((*maze).keys[(*at)[i]].symbol);
	}
	ret.push('|');
	for i in 0..keys2.len() {
		ret.push(keys2[i]);
	}
	return ret;
}

fn dijkstra_b(maze:&mut Maze, origins:&Vec<usize>)->usize {
	let mut frontier:HashMap<String, DNodeB> = HashMap::new();
	let mut frontier_next:HashMap<String, DNodeB> = HashMap::new();
	let mut explored:HashMap<String, DNodeB> = HashMap::new();
	let mut candidates:HashMap<String, usize> = HashMap::new();
	
	let mut start = DNodeB{at:Vec::new(), keys:Vec::new(), dist:0};
	for i in 0..(*origins).len() {
		start.at.push((*origins)[i]);
		start.keys.push((*origins)[i]);
	}
	frontier_next.insert(keynodeindex(maze, &(start.keys), &(start.at)), start);
	
	while frontier_next.len() > 0 {
		frontier.clear();
		for key in frontier_next.keys() {
			let node = frontier_next.get(key).unwrap();
			let node2 = (*node).clone();
			frontier.insert(key.to_string(), node2);
		}
		frontier_next.clear();
		
		for key in frontier.keys() {
			//println!("Key {}", key);
			let node = frontier.get(key).unwrap();
			if (*node).keys.len() == (*maze).keys.len() {
				if let Some(candidate) = candidates.get_mut(key) {
					if (*candidate) > (*node).dist {
						*candidate = (*node).dist;
					}
				}
				else {
					candidates.insert(key.to_string(), (*node).dist);
				}
			}
			
			// add to explored or update
			if let Some(explored_node) = explored.get_mut(key) {
				if (*explored_node).dist > (*node).dist {
					(*explored_node).keys.clear();
					(*explored_node).at.clear();
					for i in 0..(*node).keys.len() {
						(*explored_node).keys.push((*node).keys[i]);
					}
					for i in 0..(*node).at.len() {
						(*explored_node).at.push((*node).at[i]);
					}
					(*explored_node).dist = (*node).dist;
				}
			}
			else {
				let new_node = (*node).clone();
				explored.insert(key.to_string(), new_node);
			}
			
			// add all next steps from all positions
			for p in 0..(*node).at.len() {
				for k in 0..(*maze).keys.len() {
					let mut present = false;
					for j in 0..(*node).keys.len() {
						if (*node).keys[j] == k {
							present = true;
							break;
						}
					}
					if present {
						continue;
					}
					
					let curr_key = (*node).at[p];
					// if not accessible from current position
					if (*maze).cached.get(&curr_key).unwrap().get(&k).unwrap().dist < 0 {
						continue;
					}
					
					// if not accessible with current keys
					let required_keys = (*maze).cached.get(&curr_key).unwrap().get(&k).unwrap().keys.clone();
					if intersect_count(&((*node).keys), &required_keys) < required_keys.len() {
						continue;
					}
					
					let mut new_keys = (*node).keys.clone();
					new_keys.push(k);
					let mut new_at = (*node).at.clone();
					new_at[p] = k;
					let new_keys_index = keynodeindex(maze, &new_keys, &new_at);
					let new_dist = (*node).dist + ((*maze).cached.get(&curr_key).unwrap().get(&k).unwrap().dist as usize);
					
					// if previously explored and not shorter
					if explored.contains_key(&new_keys_index) && explored.get(&new_keys_index).unwrap().dist < new_dist {
						continue;
					}
					
					// if previously added to the frontier
					if frontier_next.contains_key(&new_keys_index) && frontier_next.get(&new_keys_index).unwrap().dist < new_dist {
						continue;
					}
					
					// add to frontier
					frontier_next.insert(new_keys_index, DNodeB{at:new_at, keys:new_keys, dist:new_dist});
					
				}
			}
				
		}
	}
	let mut min_dist = 0;
	for candidate_key in candidates.keys() {
		let candidate = candidates.get(candidate_key).unwrap();
		if min_dist == 0 || min_dist > *candidate {
			min_dist = *candidate;
		}
	}
	
	return min_dist;
}

fn exploredindex(maze: &mut Maze, x: usize, y:usize)->usize {
	return ((*maze).width * y) + x;
}

fn dijkstra_a(maze: &mut Maze, start_x:usize, start_y:usize, end_x:usize, end_y:usize, doors:&mut Vec<usize>, keys:&mut Vec<usize>, ret_doors_keys:bool)->i64 {
	let mut explored:HashMap<usize, DNode> = HashMap::new();
	
	let mut frontier:HashMap<usize,DNode> = HashMap::new();
	let mut frontier_next:HashMap<usize,DNode> = HashMap::new();
	
	frontier_next.insert(exploredindex(maze, start_x, start_y), DNode{x:start_x, y:start_y, dist:0, parent_x:start_x, parent_y:start_y});
	
	let dest_key = exploredindex(maze, end_x, end_y);
	
	while frontier_next.len() > 0 {
		
		frontier.clear();
		for key in frontier_next.keys() {
			let node = frontier_next.get(key).unwrap();
			let new_node = (*node).clone();
			frontier.insert(*key, new_node);
		}
		frontier_next.clear();
		
		for key in frontier.keys() {
			let node = frontier.get(key).unwrap();
			let exploredindex1 = exploredindex(maze, (*node).x, (*node).y);
			if explored.contains_key(&exploredindex1) {
				let last_dist = explored.get(&exploredindex1).unwrap().dist;
				if (*node).dist < last_dist {
					let node2 = explored.get_mut(&exploredindex1).unwrap();
					(*node2).dist = (*node).dist;
					(*node2).parent_x = (*node).parent_x;
					(*node2).parent_y = (*node).parent_y;
				}
			}
			else {
				let new_node = (*node).clone();
				explored.insert(exploredindex1, new_node);
			}
			
			let mut xd:i64 = 0;
			let mut yd:i64 = 0;
			for i in 0..4 {
				if i == 0 {
					xd = -1; yd = 0;
				} else if i == 1 {
					xd = 1; yd = 0;
				} else if i == 2 {
					xd = 0; yd = 1;
				} else if i == 3 {
					xd = 0; yd = -1;
				}	
				
				let x1 = (*node).x as i64 + xd;
				let y1 = (*node).y as i64 + yd;
				if x1 < 0 || x1 >= (*maze).width as i64 || y1 < 0 || y1 >= (*maze).height as i64 {
					continue;
				}
				else {
					if (*maze).grid[y1 as usize][x1 as usize].obstacle  {
						continue;
					}
					let index = exploredindex(maze, x1 as usize, y1 as usize);
					let new_dist = (*node).dist + 1;
					if  explored.contains_key(&index) && explored.get(&index).unwrap().dist <= new_dist {
						continue;
					}
					
					if frontier_next.contains_key(&index) && frontier_next.get(&index).unwrap().dist <= new_dist {
						continue;
					}
					
					frontier_next.insert(index, DNode{x:x1 as usize, y:y1 as usize, dist:new_dist, parent_x:(*node).x, parent_y:(*node).y});
					
				}
				
			}
		}
		
		if explored.contains_key(&dest_key) {
			let end_node = explored.get(&dest_key).unwrap();
			if ret_doors_keys {
				let mut curr_x = end_node.parent_x;
				let mut curr_y = end_node.parent_y;
				while !(curr_x == start_x && curr_y == start_y) {
					if (*maze).grid[curr_y][curr_x].key_index >= 0 {
						(*keys).push((*maze).grid[curr_y][curr_x].key_index as usize);
					}
					if (*maze).grid[curr_y][curr_x].door_index >= 0 {
						(*doors).push((*maze).grid[curr_y][curr_x].door_index as usize);
					}
					let index = exploredindex(maze, curr_x, curr_y);
					let trace = explored.get(&index).unwrap();
					curr_x = trace.parent_x;
					curr_y = trace.parent_y;
				}				
			}
			return end_node.dist as i64;
		}
	}
	return -1;
}

fn read_maze(input: Vec<String>, maze:&mut Maze)->usize {
	(*maze).width = input[0].len();
	(*maze).height = input.len();
	
	// read origin, obstacles, doors and keys
	for y in 0..(*maze).height {
		(*maze).grid.push(Vec::new());
		for x in 0..(*maze).width {
			let byte = input[y].as_bytes()[x];
			match byte {
				35=>(*maze).grid[y].push(MazeNode{obstacle:true, door_index: -1, key_index: -1}),
				46=>(*maze).grid[y].push(MazeNode{obstacle:false, door_index: -1, key_index: -1}),
				65..=90=> {(*maze).doors.push(Door{_x:x,_y:y,symbol:(byte as char),key_index:0}); (*maze).grid[y].push(MazeNode{obstacle:false, door_index: ((*maze).doors.len() - 1) as i64, key_index: -1}); },
				97..=122=> {(*maze).keys.push(Key{x:x,y:y,symbol:((byte-32) as char)}); (*maze).grid[y].push(MazeNode{obstacle:false, door_index: -1, key_index: ((*maze).keys.len() - 1) as i64}); },
				_=>{(*maze).keys.push(Key{x:x, y:y, symbol:(byte as char)}); (*maze).grid[y].push(MazeNode{obstacle:false, door_index: -1, key_index: ((*maze).keys.len() - 1) as i64});},
			}
		}
	}
	
	// quick lookup for door/key correspondance
	for i in 0..(*maze).doors.len() {
		for j in 0..(*maze).keys.len() {
			if (*maze).keys[j].symbol == (*maze).doors[i].symbol {
				(*maze).doors[i].key_index = j;
				break;
			}
		}
	}
	
	// cache distances between each key
	
	for i in 0..(*maze).keys.len() {
		(*maze).cached.insert(i, HashMap::new());
	}
	
	for i in 0..(*maze).keys.len() {
		for j in 0..(*maze).keys.len() {
			if j == i {
				continue;
			}
			let mut doors = Vec::new();
			let mut keys = Vec::new();
			let dist = dijkstra_a(maze, (*maze).keys[i].x, (*maze).keys[i].y, (*maze).keys[j].x,  (*maze).keys[j].y, &mut doors, &mut keys, true);
			let mut doorkeys:Vec<usize> = Vec::new();
			for k in 0..doors.len() {
				doorkeys.push((*maze).doors[doors[k]].key_index);
			}
			(*maze).cached.get_mut(&i).unwrap().insert(j, CachedPath{dist: dist, keys: doorkeys});
		}
	}
	
	let mut first_keys:Vec<usize> = Vec::new();
	for i in 0..(*maze).keys.len() {
		if (*maze).keys[i].symbol == '@' {
			first_keys.push(i);
		}
	}
	
	return dijkstra_b(maze, &first_keys);
}

pub fn run(file_path:&str) {
	let mut maze = Maze{grid:Vec::new(), keys:Vec::new(), doors:Vec::new(), cached:HashMap::new(), height: 0, width: 0};
	let mut maze2 = Maze{grid:Vec::new(), keys:Vec::new(), doors:Vec::new(), cached:HashMap::new(), height: 0, width: 0};
	let vec = super::utility::util_fread(file_path);
	let mut vec2:Vec<String> = Vec::new();
	
	
	let mut ox = 0;
	let mut oy = 0;
	
	if vec.len() == 0 {
		println!("Input not read properly");
		return;
	}
	
	// test if maze is set up for part B
	for line in 0..vec.len() {
		let bytes = vec[line].as_bytes();
		for pos in 0..bytes.len() {
			if bytes[pos] == '@' as u8 {
				ox = pos;
				oy = line;
			}
		}
	}
	
	let mut has_part_b = true;
	if ox + 1 >= vec[0].len() || (ox as i64 - 1) < 0 || oy + 1 >= vec.len() || (oy as i64 - 1) < 0 {
		has_part_b = false;
	}	
	else {
		for y in oy-1..=oy+1 {
			let bytes = vec[y].as_bytes();
			if y == oy-1 && (bytes[ox-1] != '.' as u8 || bytes[ox] != '.' as u8 || bytes[ox+1] != '.' as u8) {
				has_part_b = false;
				break;
			}
			else if y == oy && (bytes[ox-1] != '.' as u8 || bytes[ox] != '@' as u8 || bytes[ox+1] != '.' as u8) {
				has_part_b = false;
				break;
			}
			else if y == oy+1 && (bytes[ox-1] != '.' as u8 || bytes[ox] != '.' as u8 || bytes[ox+1] != '.' as u8) {
				has_part_b = false;
				break;
			}
		}
	}
	
	if has_part_b {
		for y in 0..vec.len() {
			let mut line = String::from("");
			let bytes = vec[y].as_bytes();
			for x in 0..vec[y].len() {
				if  (x == ox - 1 && y == oy - 1) ||
					(x == ox + 1 && y == oy - 1) ||
					(x == ox - 1 && y == oy + 1) ||
					(x == ox + 1 && y == oy + 1) {
					line.push('@');
				} else if (x == ox && y == oy - 1) ||
					(x == ox && y == oy + 1) ||
					(x == ox - 1 && y == oy) ||
					(x == ox + 1 && y == oy) ||
					(x == ox && y == oy) {
					line.push('#');
					
				}
				else {
					line.push(bytes[x] as char);
				}
			}
			vec2.push(line);
		}
	}
	
	
	let result_a = read_maze(vec, &mut maze);
	println!("Result A: {}", result_a);
	if has_part_b {
		
		
		let result_b = read_maze(vec2, &mut maze2);
		println!("Result B: {}", result_b);
	}
}