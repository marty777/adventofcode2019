// day 24
use std::collections::HashMap;

struct BugGrid {
	grid:Vec<Vec<bool>>,
	width:usize,
	height:usize,
}

fn grid_biodiversity(grid:&mut BugGrid)->u64 {
	if (*grid).width * (*grid).height >= 64 {
		return 0;
	}
	let mut accumulator:u64 = 0;
	let pow = 1;
	for y in 0..(*grid).height {
		for x in 0..(*grid).width {
			if (*grid).grid[y][x] {
				accumulator += pow << (y * (*grid).width + x);
			}
		}
	}
	return accumulator;
}

fn run_grid(grid:&mut BugGrid) {
	let mut adjacency:Vec<Vec<usize>> = Vec::new();
	for y in 0..(*grid).height {
		adjacency.push(Vec::new());
		for x in 0..(*grid).width {
			let mut count = 0;
			if y > 0 && (*grid).grid[y-1][x] {
				count += 1;
			}
			if x > 0 && (*grid).grid[y][x-1]{
				count += 1;
			}
			if y < (*grid).height-1 && (*grid).grid[y+1][x] {
				count += 1;
			}
			if x < (*grid).width-1 && (*grid).grid[y][x+1] {
				count += 1;
			}
			adjacency[y].push(count);
		}
		
	}
	for y in 0..(*grid).height {
		for x in 0..(*grid).width {
			if (*grid).grid[y][x] && adjacency[y][x] != 1 {
				(*grid).grid[y][x] = false;
			}
			else if !(*grid).grid[y][x] && (adjacency[y][x] == 1 || adjacency[y][x] == 2) {
				(*grid).grid[y][x] = true;
			}
		}
	}
}

fn run_grids(grids: &mut Vec<BugGrid>) {
	let mut adjacency:Vec<Vec<Vec<usize>>> = Vec::new();
	adjacency.push(Vec::new());
	for i in 1..(*grids).len()-1 { // we don't compute the outer two layers of pre-allocated grid
		adjacency.push(Vec::new());
		for y in 0..(*grids)[i].grid.len() {
			adjacency[i].push(Vec::new());
			for x in 0..(*grids)[i].grid[y].len() {
				let mut count:usize = 0;
				// north
				if y == 0 {
					count += (*grids)[i-1].grid[1][2] as usize;
				}
				else if  y == 3 && x == 2 {
					count += (*grids)[i+1].grid[4][0] as usize;
					count += (*grids)[i+1].grid[4][1] as usize;
					count += (*grids)[i+1].grid[4][2] as usize;
					count += (*grids)[i+1].grid[4][3] as usize;
					count += (*grids)[i+1].grid[4][4] as usize;
				}
				else {
					count += (*grids)[i].grid[y-1][x] as usize;
				}
				
				// east
				if x == 4 {
					count += (*grids)[i-1].grid[2][3] as usize;
				}
				else if y == 2 && x == 1 {
					count += (*grids)[i+1].grid[0][0] as usize;
					count += (*grids)[i+1].grid[1][0] as usize;
					count += (*grids)[i+1].grid[2][0] as usize;
					count += (*grids)[i+1].grid[3][0] as usize;
					count += (*grids)[i+1].grid[4][0] as usize;
				}
				else {
					count += (*grids)[i].grid[y][x+1] as usize;
				}
				
				// south 
				if y == 4 {
					count += (*grids)[i-1].grid[3][2] as usize;
				}
				else if y == 1 && x == 2 {
					count += (*grids)[i+1].grid[0][0] as usize;
					count += (*grids)[i+1].grid[0][1] as usize;
					count += (*grids)[i+1].grid[0][2] as usize;
					count += (*grids)[i+1].grid[0][3] as usize;
					count += (*grids)[i+1].grid[0][4] as usize;
				}
				else {
					count += (*grids)[i].grid[y+1][x] as usize;
				}
				
				// west 
				if x == 0 {
					count += (*grids)[i-1].grid[2][1] as usize;
				}
				else if y == 2 && x == 3 {
					count += (*grids)[i+1].grid[0][4] as usize;
					count += (*grids)[i+1].grid[1][4] as usize;
					count += (*grids)[i+1].grid[2][4] as usize;
					count += (*grids)[i+1].grid[3][4] as usize;
					count += (*grids)[i+1].grid[4][4] as usize;
				}
				else {
					count += (*grids)[i].grid[y][x-1] as usize;
				}
				
				adjacency[i][y].push(count);
			}
		}
	}
	for i in 1..(*grids).len() - 1 {
		for y in 0..(*grids)[i].height {
			for x in 0..(*grids)[i].width {
				if x == 2 && y == 2 {
					(*grids)[i].grid[y][x] = false;
					continue;
				}
				if (*grids)[i].grid[y][x] && adjacency[i][y][x] != 1 {
					(*grids)[i].grid[y][x] = false;
				}
				else if !(*grids)[i].grid[y][x] && (adjacency[i][y][x] == 1 || adjacency[i][y][x] == 2) {
					(*grids)[i].grid[y][x] = true;
				}
			}
		}
	}
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut grid = BugGrid{grid:Vec::new(), width:0, height:0};
	grid.height = vec.len();
	grid.width = vec[0].len();
	for y in 0..vec.len() {
		grid.grid.push(Vec::new());
		let bytes = vec[y].as_bytes();
		for x in 0..bytes.len() {
			if bytes[x] == '.' as u8 {
				grid.grid[y].push(false);
			}
			else {
				grid.grid[y].push(true);
			}
		}
	}
	let mut biodiversity:HashMap<u64, usize> = HashMap::new();
	biodiversity.insert(grid_biodiversity(&mut grid), 1);
	loop {
		run_grid(&mut grid);
		let curr_biodiversity = grid_biodiversity(&mut grid);
		if let Some(biodiversity_count) = biodiversity.get_mut(&curr_biodiversity) {
			*biodiversity_count += 1;
			if *biodiversity_count == 2 {
				println!("Result A: {}", curr_biodiversity);
				break;
			}
		}
		else {
			biodiversity.insert(curr_biodiversity, 1);
		}
	}
	
	let max_grids = 120;
	let iterations = 200;
	let mut grids:Vec<BugGrid> = Vec::new();
	for i in 0..2*max_grids + 1 {
		grids.push(BugGrid{grid:Vec::new(), width:grid.width, height:grid.height});
		// level 0
		if i == max_grids {
			for y in 0..vec.len() {
				grids[i].grid.push(Vec::new());
				let bytes = vec[y].as_bytes();
				for x in 0..bytes.len() {
					if bytes[x] == '.' as u8 {
						grids[i].grid[y].push(false);
					}
					else {
						grids[i].grid[y].push(true);
					}
				}
			}
		}
		else {
			for y in 0..grids[i].height {
				grids[i].grid.push(Vec::new());
				for _x in 0..grids[i].width {
					grids[i].grid[y].push(false);
				}
			}
		}
	}
	
	for _i in 0..iterations {
		run_grids(&mut grids);
	}
		
	let mut bug_count = 0;
	for i in 0..grids.len() {
		for y in 0..grids[i].height {
			for x in 0..grids[i].width {
				bug_count += grids[i].grid[y][x] as usize;
			}
		}
	}
	
	println!("Result B: {}", bug_count);
}