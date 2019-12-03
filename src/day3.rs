// day 3 

struct WirePath {
	dir: char,
	len: i32,
}

pub fn run(file_path:&str) {
		let vec = super::utility::util_fread(file_path);
		let dirs1_str = vec[0].split(","); 
		let dirs2_str = vec[1].split(","); 
		let mut path1: Vec<WirePath> = Vec::new();
		let mut path2: Vec<WirePath> = Vec::new();
		for dirstr in dirs1_str {
			let substr: String = dirstr.chars().skip(1).take(dirstr.chars().count() - 1).collect();
			let len = substr.parse::<i32>().unwrap();
			let dir = dirstr.chars().next().unwrap();
			path1.push( WirePath{dir:dir,len:len});
		}
		for dirstr in dirs2_str {
			let substr: String = dirstr.chars().skip(1).take(dirstr.chars().count() - 1).collect();
			let len = substr.parse::<i32>().unwrap();
			let dir = dirstr.chars().next().unwrap();
			path2.push(WirePath{dir:dir,len:len});
		}
		
		// determine bounds
		let mut max_x:i32 = 0;
		let mut max_y:i32 = 0;
		let mut min_x:i32 = 0;
		let mut min_y:i32 = 0;
		
		let mut pos_x:i32 = 0;
		let mut pos_y:i32 = 0;
		
		for i in 0..path1.len() {
			match path1[i].dir {
			'U'=>pos_y+=path1[i].len,
			'D'=>pos_y-=path1[i].len,
			'L'=>pos_x-=path1[i].len,
			'R'=>pos_x+=path1[i].len,
			_=>continue,
			}
			if pos_x > max_x {
				max_x = pos_x;
			}
			if pos_x < min_x {
				min_x = pos_x;
			}
			if pos_y > max_y {
				max_y = pos_y;
			}
			if pos_y < min_y {
				min_y = pos_y;
			}
		}
		
		pos_x = 0;
		pos_y = 0;
		for i in 0..path2.len() {
			match path2[i].dir {
			'U'=>pos_y+=path2[i].len,
			'D'=>pos_y-=path2[i].len,
			'L'=>pos_x-=path2[i].len,
			'R'=>pos_x+=path2[i].len,
			_=>continue,
			}
			if pos_x > max_x {
				max_x = pos_x;
			}
			if pos_x < min_x {
				min_x = pos_x;
			}
			if pos_y > max_y {
				max_y = pos_y;
			}
			if pos_y < min_y {
				min_y = pos_y;
			}
		}
		
		let dim_x = max_x - min_x + 1;
		let dim_y = max_y - min_y + 1;
		let mut grid = vec![-1; (dim_x * dim_y) as usize];
		 pos_x = (max_x - min_x) - max_x;
		 pos_y = (max_y - min_y) - max_y;
		 
		let mut step_count = 0;
		 for i in 0..path1.len() {
			 for _k in 0..path1[i].len {
				 match path1[i].dir {
				 'U'=> pos_y += 1,
				 'D'=>pos_y -= 1,
				 'L'=>pos_x -= 1,
				 'R'=>pos_x += 1,
				 _=>continue,
				 }
				 step_count += 1;
				 if grid[((pos_y * dim_x) + pos_x) as usize] < 0 {
					grid[((pos_y * dim_x) + pos_x) as usize]= step_count;
				 }
			 }
		 } 
		 
		 pos_x = (max_x - min_x) - max_x;
		 pos_y = (max_y - min_y) - max_y;
		 step_count = 0;
		 let mut min_manhattan_distance:i32 = (dim_x) + (dim_y);
		 let mut min_wire_dist:i32 = (dim_x) * (dim_y);
		 for i in 0..path2.len() {
			 for _k in 0..path2[i].len {
				 match path2[i].dir {
				 'U'=> pos_y += 1,
				 'D'=>pos_y -= 1,
				 'L'=>pos_x -= 1,
				 'R'=>pos_x += 1,
				 _=>continue,
				 }
				 step_count+=1;
				 if  grid[((pos_y * dim_x) + pos_x) as usize] >= 0 {
					let dist = (pos_x - ((max_x - min_x) - max_x)).abs() + (pos_y - ((max_y - min_y) - max_y)).abs();
					let wire_dist = step_count + grid[((pos_y * dim_x) + pos_x) as usize];
					if dist < min_manhattan_distance {
						min_manhattan_distance = dist;
					}
					if wire_dist < min_wire_dist {
						min_wire_dist = wire_dist;
					}
				 }
			 }
		 } 
		println!("Result A: {}", min_manhattan_distance);
		println!("Result B: {}", min_wire_dist);
}