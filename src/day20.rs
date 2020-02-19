// day 20
use std::collections::HashMap;

#[derive(PartialEq)]
enum Dir {
	North,
	East,
	South,
	West
}

#[derive(Copy,Clone, Hash)]
struct Coord {
	x: usize,
	y: usize,
}

impl PartialEq for Coord {
	fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Coord {}

#[derive(Copy,Clone, Hash)]
struct LevelCoord {
	x:usize,
	y:usize,
	level:usize,
}

impl PartialEq for LevelCoord {
	fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.level == other.level;
    }
}

impl Eq for LevelCoord {}

#[derive(Copy,Clone, Hash)]
struct LevelPortalIndex {
	index:usize,
	level:usize,
}

impl PartialEq for LevelPortalIndex {
	fn eq(&self, other: &Self) -> bool {
        return self.index == other.index && self.level == other.level;
    }
}

impl Eq for LevelPortalIndex {}

struct MazeEdge {
	x: usize,
	y: usize,
	passable:bool,
	up:bool,
	down:bool,
}

struct PortalPosition {
	symbol_a:char,
	symbol_b:char,
	start_coord:Coord,
	horizontal:bool,
	entry_coord:Coord,
	entry_dir:Dir,
	external:bool,
}

struct MazeNode {
	coord: Coord,
	n:MazeEdge,
	e:MazeEdge,
	s:MazeEdge,
	w:MazeEdge,
}

struct Maze {
	grid: Vec<Vec<MazeNode>>,
	portal_positions:Vec<PortalPosition>,
	cached_distances:Vec<Vec<i64>>,
	width: usize,
	height: usize,
}

struct DNode {
	positions:Vec<usize>,
	dist: usize,
}

fn dijkstra_c(maze:&Maze, start_index:usize, end_index:usize, levels:bool)->i64 {
	let mut frontier:HashMap<LevelPortalIndex, DNode> = HashMap::new();
	let mut frontier_next:HashMap<LevelPortalIndex, DNode> = HashMap::new();
	let mut explored:HashMap<LevelPortalIndex, DNode> = HashMap::new();
	
	let mut start_coord = (*maze).portal_positions[start_index].entry_coord;
	let mut end_coord = (*maze).portal_positions[end_index].entry_coord;
	
	start_coord.x -= 2;
	start_coord.y -= 2;
	end_coord.x -= 2;
	end_coord.y -= 2;
	
	let endlevelindex = LevelPortalIndex{index:end_index, level:0};
	
	frontier_next.insert(LevelPortalIndex{index:start_index, level:0}, DNode{positions:Vec::new(), dist:0});

	
	while frontier_next.len() > 0 {
		// copy to frontier
		frontier.clear();
		for key in frontier_next.keys() {
			let node1 =  frontier_next.get(key).unwrap();
			let mut node2 = DNode{positions:Vec::new(), dist:(*node1).dist};
			for i in 0..(*node1).positions.len() {
				node2.positions.push((*node1).positions[i]);
			}
			frontier.insert(*key, node2);
		}
		frontier_next.clear();
		
		// try all possible next moves
		for coord in frontier.keys() {
			
			let curr_node = frontier.get(coord).unwrap();
			let curr_dist = curr_node.dist;
			// update explored for coord
			if !explored.contains_key(coord) || 
				(explored.contains_key(coord) && (*explored.get(coord).unwrap()).dist > curr_dist) {
				let mut next_node = DNode{positions:Vec::new(), dist: curr_dist};
				for i in 0..(*curr_node).positions.len() {
					next_node.positions.push((*curr_node).positions[i]);
				}
				explored.insert(*coord, next_node);
			}
			
			// follow portal
			let corresponding_index = matching_portal_index(maze, coord.index);
			if corresponding_index >= 0 {
				let mut coord2 = LevelPortalIndex{index:corresponding_index as usize, level:coord.level};
				let next_dist = curr_dist + 1;
				let mut valid_move = true;
				if levels {
					// can't go through an exterior portal if at level 0
					if coord.level == 0 && (*maze).portal_positions[coord.index].external {
						valid_move = false;
					}
					else {
						if (*maze).portal_positions[coord.index].external {
							coord2.level -= 1;
						}	
						else {
							coord2.level += 1;
						}
					}
				}
				if valid_move && 
					(!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()).dist > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()).dist > next_dist)) {
					let mut next_node = DNode{positions:Vec::new(), dist:next_dist};
					for i in 0..curr_node.positions.len() {
						next_node.positions.push(curr_node.positions[i]);
					}
					next_node.positions.push(corresponding_index as usize);
					frontier_next.insert(coord2, next_node);
				}
			}
			
			// move across to other reachable portals
			for i in 0..(*maze).portal_positions.len() {
				if i == coord.index || (*maze).cached_distances[coord.index][i] < 0 {
					continue;
				}
				let coord2 = LevelPortalIndex{index: i, level:coord.level};
				let next_dist = curr_dist + (*maze).cached_distances[coord.index][i] as usize;
				if (!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()).dist > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()).dist > next_dist)) {
					let mut next_node = DNode{positions:Vec::new(), dist:next_dist};
					for i in 0..curr_node.positions.len() {
						next_node.positions.push(curr_node.positions[i]);
					}
					next_node.positions.push(i);
					frontier_next.insert(coord2, next_node);
				}
			}
		}
		
		// if we've reached the end, there still may be unexplored paths that could be shorter.
		// wait until all frontier distances are greater than the shortest end distance before returning result
		if explored.contains_key(&endlevelindex) {
			let end_dist = (*explored.get(&endlevelindex).unwrap()).dist;
			let mut min_frontier_dist = 0;
			for key in frontier_next.keys() {
				if min_frontier_dist == 0 || (*frontier_next.get(key).unwrap()).dist < min_frontier_dist {
					min_frontier_dist = (*frontier_next.get(key).unwrap()).dist;
				}
			}
			if min_frontier_dist >= end_dist {
				return end_dist as i64;
			}
		}
	}
	
	if explored.contains_key(&endlevelindex) {
		let dist = (*explored.get(&endlevelindex).unwrap()).dist;
		return dist as i64;
	}
	
	return -1;
}

// path between two points on map, following no portals. used to build cached distances
fn dijkstra_a(maze:&Maze, start:Coord, end:Coord)->i64 {
	let mut frontier:HashMap<LevelCoord, usize> = HashMap::new(); 
	let mut frontier_next:HashMap<LevelCoord, usize> = HashMap::new();
	let mut explored:HashMap<LevelCoord, usize> = HashMap::new();
	
	frontier_next.insert(LevelCoord{x:start.x, y:start.y, level:0}, 0);
	let end_coord = LevelCoord{x:end.x, y:end.y, level:0};
	while frontier_next.len() > 0 {
		// copy to frontier
		frontier.clear();
		for key in frontier_next.keys() {
			frontier.insert(*key, *(frontier_next.get(key).unwrap()));
		}
		frontier_next.clear();
		
		// try all possible next moves
		for coord in frontier.keys() {
			let curr_dist:usize = *frontier.get(coord).unwrap();
			// update explored for coord
			if !explored.contains_key(coord) || (explored.contains_key(coord) && *(explored.get(coord).unwrap()) > curr_dist) {
				explored.insert(*coord, curr_dist);
			}
			
			let next_dist = curr_dist + 1;
			if (*maze).grid[(*coord).y][(*coord).x].n.passable {
				let coord2 = LevelCoord{x: (*maze).grid[(*coord).y][(*coord).x].n.x, y:(*maze).grid[(*coord).y][(*coord).x].n.y, level:(*coord).level};
				let mut valid_move = true;
				// don't follow portals
				if (*maze).grid[(*coord).y][(*coord).x].n.up || (*maze).grid[(*coord).y][(*coord).x].n.down {
					valid_move = false;
				}
				if valid_move && 
					(!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()) > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()) > next_dist)) {
					frontier_next.insert(coord2, next_dist);
				}
			}
			if (*maze).grid[(*coord).y][(*coord).x].e.passable {
				let coord2 = LevelCoord{x: (*maze).grid[(*coord).y][(*coord).x].e.x, y:(*maze).grid[(*coord).y][(*coord).x].e.y, level:(*coord).level};
				let mut valid_move = true;
				if (*maze).grid[(*coord).y][(*coord).x].e.up || (*maze).grid[(*coord).y][(*coord).x].e.down {
					valid_move = false;
				}
				if valid_move && 
					(!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()) > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()) > next_dist)) {
					frontier_next.insert(coord2, next_dist);
				}
			}
			if (*maze).grid[(*coord).y][(*coord).x].s.passable {
				let coord2 = LevelCoord{x: (*maze).grid[(*coord).y][(*coord).x].s.x, y:(*maze).grid[(*coord).y][(*coord).x].s.y, level:(*coord).level};
				let mut valid_move = true;
				if (*maze).grid[(*coord).y][(*coord).x].s.up || (*maze).grid[(*coord).y][(*coord).x].s.down {
					valid_move = false;
				}
				if valid_move && 
					(!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()) > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()) > next_dist)) {
					frontier_next.insert(coord2, next_dist);
				}
			}
			if (*maze).grid[(*coord).y][(*coord).x].w.passable {
				let coord2 = LevelCoord{x: (*maze).grid[(*coord).y][(*coord).x].w.x, y:(*maze).grid[(*coord).y][(*coord).x].w.y, level:(*coord).level};
				let mut valid_move = true;
				if (*maze).grid[(*coord).y][(*coord).x].w.up || (*maze).grid[(*coord).y][(*coord).x].w.down {
					valid_move = false;
				}
				if valid_move && 
					(!frontier_next.contains_key(&coord2) || 
					(frontier_next.contains_key(&coord2) && (*frontier_next.get(&coord2).unwrap()) > next_dist)) &&
					(!explored.contains_key(&coord2) || 
					(explored.contains_key(&coord2) && (*explored.get(&coord2).unwrap()) > next_dist)) {
					frontier_next.insert(coord2, next_dist);
				}
			}
		}
		
		if explored.contains_key(&end_coord) {
			let end_dist = explored.get(&end_coord).unwrap();
			return *end_dist as i64;
		}
	}
	
	
	return -1;
}

fn matching_portal_index(maze:&Maze, index:usize)->i64 {
	if index >= (*maze).portal_positions.len() {
		return -1;
	}
	for i in 0..(*maze).portal_positions.len() {
		if i == index {
			continue;
		}
		if (*maze).portal_positions[i].symbol_a == (*maze).portal_positions[index].symbol_a && (*maze).portal_positions[i].symbol_b == (*maze).portal_positions[index].symbol_b {
			return i as i64;
		}
	}
	
	return -1;
}

fn read_maze(input: Vec<String>, maze:&mut Maze)->usize {
	
	if input.len() == 0 {
		return 0;
	}
	
	// find all the portals
	let char_width = input[0].len();
	let char_height = input.len();
	let mut chars:Vec<Vec<char>> = Vec::new();
	for y in 0..input.len() {
		chars.push(Vec::new());
		let line = input[y].as_bytes();
		for x in 0..line.len() {
			chars[y].push(line[x] as char);
		}
	}
	
	let mut start_portal_index = 0;
	let mut end_portal_index = 0;
	
	for y in 0..chars.len() {
		for x in 0..chars[y].len() {
			if chars[y][x] >= 'A' && chars[y][x] <= 'Z' {
				// see if we've already encountered this portal. If not, add it to the list
				let mut found = false;
				for i in 0..(*maze).portal_positions.len() {
					if (x == (*maze).portal_positions[i].start_coord.x && y == (*maze).portal_positions[i].start_coord.y) ||
						((*maze).portal_positions[i].horizontal && (*maze).portal_positions[i].start_coord.x + 1 == x && (*maze).portal_positions[i].start_coord.y == y) ||
						(!(*maze).portal_positions[i].horizontal && (*maze).portal_positions[i].start_coord.x == x && (*maze).portal_positions[i].start_coord.y == y+1) {
							found = true;
							break;
						}
				}
				if !found {
					let temp_x:i64 = x as i64;
					let temp_y:i64 = y as i64;
					let mut external = false;
					if x < 2 || y < 2 || x >= chars[0].len() - 2 || y >= chars.len() - 2 {
						external = true;
					}
					
					
					// vertical down
					if temp_y + 2 < char_height as i64 && chars[y+1][x] >= 'A' && chars[y+1][x] <= 'Z' && chars[y+2][x] == '.' {
						(*maze).portal_positions.push(PortalPosition{symbol_a:chars[y][x], symbol_b:chars[y+1][x], start_coord:Coord{x:x,y:y}, horizontal:false, entry_coord:Coord{x:x,y:y+2}, entry_dir:Dir::North, external:external});
					}
					// vertical up
					else if temp_y - 1 >= 0 && temp_y + 1 < char_height as i64 && chars[y+1][x] >= 'A' && chars[y+1][x] <= 'Z' && chars[y-1][x] == '.'{
						(*maze).portal_positions.push(PortalPosition{symbol_a:chars[y][x], symbol_b:chars[y+1][x], start_coord:Coord{x:x,y:y}, horizontal:false, entry_coord:Coord{x:x,y:y-1}, entry_dir:Dir::South, external:external})
					}
					// horizontal right 
					else if temp_x + 2 < char_width as i64 && chars[y][x+1] >= 'A' && chars[y][x+1] <= 'Z' && chars[y][x+2] == '.' {
						(*maze).portal_positions.push(PortalPosition{symbol_a:chars[y][x], symbol_b:chars[y][x+1], start_coord:Coord{x:x, y:y}, horizontal:true, entry_coord:Coord{x:x+2, y:y}, entry_dir:Dir::West, external:external});
					}
					// horizontal left 
					else if temp_x - 1 >= 0 && temp_x + 1 < char_width as i64 && chars[y][x+1] >= 'A' && chars[y][x+1] <= 'Z' && chars[y][x-1] == '.' {
						(*maze).portal_positions.push(PortalPosition{symbol_a:chars[y][x], symbol_b:chars[y][x+1], start_coord:Coord{x:x, y:y}, horizontal:true, entry_coord:Coord{x:x-1, y:y}, entry_dir:Dir::East, external:external});
					}
				}
			}
		}
	}
	
	for i in 0..(*maze).portal_positions.len() {
		if (*maze).portal_positions[i].symbol_a == 'A' && (*maze).portal_positions[i].symbol_b == 'A' {
			start_portal_index = i;
		}
		else if (*maze).portal_positions[i].symbol_a == 'Z' && (*maze).portal_positions[i].symbol_b == 'Z' {
			end_portal_index = i;
		} 
	}
	
	// build map
	// sloppily assume that the main grid will have a 2 character margin on all sides without testing it
	(*maze).width = chars[0].len() - 4;
	(*maze).height = chars.len() - 4;
	for y in 2..chars.len() - 2 {
		(*maze).grid.push(Vec::new());
		for x in 2..chars[y].len() - 2 {
			let temp_x = x - 2;
			let temp_y = y - 2;
			if chars[y][x] != '.' {
				(*maze).grid[temp_y].push(MazeNode{	coord:Coord{x:temp_x, y:temp_y}, 
													n:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
													e:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
													s:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
													w:MazeEdge{x:0,y:0,passable:false, up:false, down:false}});
			}
			else {
				// some more sloppy assumptions here. Assume a . isn't on the edge of the map unless it links to a portal.
				let mut node = MazeNode{coord:Coord{x:temp_x, y:temp_y},
										n:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
										e:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
										s:MazeEdge{x:0,y:0,passable:false, up:false, down:false},
										w:MazeEdge{x:0,y:0,passable:false, up:false, down:false}};
				
				// N
				if chars[y-1][x] == '.' {
					node.n.x = temp_x;
					node.n.y = temp_y - 1;
					node.n.passable = true;
				}
				else if chars[y-1][x] >= 'A' && chars[y-1][x] <= 'Z' {
					// find possible portal.
					for i in 0..(*maze).portal_positions.len() {
						if (*maze).portal_positions[i].entry_coord.x == x && (*maze).portal_positions[i].entry_coord.y == y && (*maze).portal_positions[i].entry_dir == Dir::North {
							let portal_index2 = matching_portal_index(maze, i);
							if portal_index2 >= 0 {
								node.n.x = (*maze).portal_positions[portal_index2 as usize].entry_coord.x - 2;
								node.n.y = (*maze).portal_positions[portal_index2 as usize].entry_coord.y - 2;
								node.n.passable = true;
								if node.coord.y == 0 {
									node.n.up = true;
								}
								else {
									node.n.down = true;
								}
								break;
							}
						}
					}
				}
				// S
				if chars[y+1][x] == '.' {
					node.s.x = temp_x;
					node.s.y = temp_y + 1;
					node.s.passable = true;
				}
				else if chars[y+1][x] >= 'A' && chars[y+1][x] <= 'Z' {
					// find possible portal.
					for i in 0..(*maze).portal_positions.len() {
						if (*maze).portal_positions[i].entry_coord.x == x && (*maze).portal_positions[i].entry_coord.y == y && (*maze).portal_positions[i].entry_dir == Dir::South {
							let portal_index2 = matching_portal_index(maze, i);
							if portal_index2 >= 0 {
								node.s.x = (*maze).portal_positions[portal_index2 as usize].entry_coord.x - 2;
								node.s.y = (*maze).portal_positions[portal_index2 as usize].entry_coord.y - 2;
								node.s.passable = true;
								if node.coord.y == (*maze).height - 1 {
									node.s.up = true;
								}
								else {
									node.s.down = true;
								}
								break;
							}
						}
					}
				}
				
				// W
				if chars[y][x-1] == '.' {
					node.w.x = temp_x - 1;
					node.w.y = temp_y;
					node.w.passable = true;
				}
				else if chars[y][x-1] >= 'A' && chars[y][x-1] <= 'Z' {
					// find possible portal.
					for i in 0..(*maze).portal_positions.len() {
						if (*maze).portal_positions[i].entry_coord.x == x && (*maze).portal_positions[i].entry_coord.y == y && (*maze).portal_positions[i].entry_dir == Dir::West {
							let portal_index2 = matching_portal_index(maze, i);
							if portal_index2 >= 0 {
								node.w.x = (*maze).portal_positions[portal_index2 as usize].entry_coord.x - 2;
								node.w.y = (*maze).portal_positions[portal_index2 as usize].entry_coord.y - 2;
								node.w.passable = true;
								if node.coord.x == 0 {
									node.w.up = true;
								}
								else {
									node.w.down = true;
								}
								break;
							}
						}
					}
				}
				
				// E
				if chars[y][x+1] == '.' {
					node.e.x = temp_x + 1;
					node.e.y = temp_y;
					node.e.passable = true;
				}
				else if chars[y][x+1] >= 'A' && chars[y][x+1] <= 'Z' {
					// find possible portal.
					for i in 0..(*maze).portal_positions.len() {
						if (*maze).portal_positions[i].entry_coord.x == x && (*maze).portal_positions[i].entry_coord.y == y && (*maze).portal_positions[i].entry_dir == Dir::East {
							let portal_index2 = matching_portal_index(maze, i);
							if portal_index2 >= 0 {
								node.e.x = (*maze).portal_positions[portal_index2 as usize].entry_coord.x - 2;
								node.e.y = (*maze).portal_positions[portal_index2 as usize].entry_coord.y - 2;
								node.e.passable = true;
								if node.coord.x == (*maze).width - 1 {
									node.e.up = true;
								}
								else {
									node.e.down = true;
								}
								break;
							}
						}
					}
				}
				
				(*maze).grid[temp_y].push(node);
			}
		}
	}
	
	// build cached distances
	for i in 0..(*maze).portal_positions.len() {
		(*maze).cached_distances.push(Vec::new());
		for j in 0..(*maze).portal_positions.len() {
			if j == i {
				(*maze).cached_distances[i].push(0);
			}
			else if (*maze).cached_distances.len() > j && (*maze).cached_distances[j].len() < i {
				let dist = (*maze).cached_distances[j][i];
				(*maze).cached_distances[i].push(dist);
			}
			else {
				let mut start_coord = (*maze).portal_positions[i].entry_coord;
				let mut end_coord = (*maze).portal_positions[j].entry_coord;
				start_coord.x -= 2;
				start_coord.y -= 2;
				end_coord.x -= 2;
				end_coord.y -= 2;
				let dist = dijkstra_a(maze, start_coord, end_coord);
				(*maze).cached_distances[i].push(dist);
			}
		}
	}
	
	let dist_a = dijkstra_c(maze, start_portal_index, end_portal_index, false);
	println!("Result A: {}", dist_a);
	let dist_b = dijkstra_c(maze, start_portal_index, end_portal_index, true);
	println!("Result B: {}", dist_b);
	
	return 0;
}

pub fn run(file_path:&str) {
	let mut maze = Maze{grid:Vec::new(), portal_positions:Vec::new(), cached_distances:Vec::new(), width:0, height:0};
	let vec = super::utility::util_fread(file_path);
	
	read_maze(vec, &mut maze);
	
	return;
}