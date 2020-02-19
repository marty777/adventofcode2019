// day 10 
use std::cmp::Ordering;
use std::f64::consts::PI;

struct Asteroid {
	x:i32,
	y:i32,
}

struct Target {
	x:i32,
	y:i32,
	bearing:f64,
	dist2:f64, // distance squared
	destroyed:bool,
}

fn cmp_bearing(a: &Target, b: &Target) -> Ordering {
    if a.bearing < b.bearing {
        return Ordering::Less;
    } else if a.bearing > b.bearing {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let x_dim:i32 = vec[0].len() as i32;
	let y_dim:i32 = vec.len() as i32;
	let mut asteroids:Vec<Asteroid> = Vec::new();
	for y in 0..vec.len() {
		let bytes = vec[y].as_bytes();
		for x in 0..bytes.len() {
			if bytes[x] == 0x23 { // #
				asteroids.push(Asteroid{x: x as i32, y:y as i32});
			}
		}
	}
	
	let mut max_asteroid_count = 0;
	let mut max_index = 0;
	
	let mut asteroid_map = vec![false; (x_dim * y_dim) as usize];
	for i in 0..asteroids.len() {
		asteroid_map[(asteroids[i].y * x_dim + asteroids[i].x) as usize] = true;
	}
	
	for i in 0..asteroids.len() {
		let mut asteroid_count = 0;
		for j in 0..asteroids.len() {
			if j == i {
				continue;
			}
			let x_vec = asteroids[j].x - asteroids[i].x;
			let y_vec = asteroids[j].y - asteroids[i].y;
			let dist2 = x_vec * x_vec + y_vec * y_vec;
			let dimdir_x = if x_vec < 0 {-1} else if x_vec == 0 {0} else {1}; 
			let dimdir_y = if y_vec < 0 {-1} else if y_vec == 0 {0} else {1}; 
			let rat:f64 = (x_vec as f64)/(y_vec as f64);
			// check all asteroids for identical vector. If closer to i, asteroid k is occluded
			let mut occluded = false;
			for k in 0..asteroids.len() {
				if k == j {
					continue;
				}
				let x_vec2 = asteroids[k].x - asteroids[i].x;
				let y_vec2 = asteroids[k].y - asteroids[i].y;
				let dist2_2 = x_vec2 * x_vec2 + y_vec2 * y_vec2;
				if dist2_2 >= dist2 {
					continue;
				}
				let dimdir_x2 = if x_vec2 < 0 {-1} else if x_vec2 == 0 {0} else {1}; 
				let dimdir_y2 = if y_vec2 < 0 {-1} else if y_vec2 == 0 {0} else {1}; 
				if dimdir_x != dimdir_x2 || dimdir_y != dimdir_y2 {
					continue;
				}
				let rat2:f64 = (x_vec2 as f64)/(y_vec2 as f64);
				if rat2 == rat {
					occluded = true;
					break;
				}
			}
			if !occluded {
				asteroid_count += 1;
			}
		}
		
		if asteroid_count > max_asteroid_count {
			max_asteroid_count = asteroid_count;
			max_index = i;
		}
	}
	
	println!("Result A: {}", max_asteroid_count);
	
	let mut target_list:Vec<Target> = Vec::new();
	for i in 0..asteroids.len() {
		if i == max_index {
			continue;
		}
		let x_vec:f64 = (asteroids[max_index].x - asteroids[i].x) as f64;
		let y_vec:f64 = (asteroids[max_index].y - asteroids[i].y) as f64;
		let angle:f64 = y_vec.atan2(x_vec)/PI;
		let dist2:f64 = x_vec * x_vec + y_vec * y_vec;
		target_list.push(Target{x:asteroids[i].x, y:asteroids[i].y, bearing:angle, dist2:dist2, destroyed:false});
	}
	
	target_list.sort_by(cmp_bearing);
	
	let mut destroyed_count = 0;
	let mut result_b_destroyed_coords = 0;
	let start_angle = 0.5;
	
	loop {
		let mut curr_targets:Vec<usize> = Vec::new();
		let mut start_index:usize = 0;
		for i in 0..target_list.len() {
			if target_list[i].bearing < start_angle {
				continue;
			}
			start_index = i;
			break;
		}
		for i in 0..target_list.len() {
			let j = (i + start_index) % target_list.len();
			if target_list[j].destroyed {
				continue;
			}
			// check all asteroids for identical vector. If closer to i, asteroid k is occluded
			let mut occluded = false;
			for k in 0..target_list.len() {
				if k == j || target_list[k].destroyed {
					continue;
				}
				if target_list[k].bearing != target_list[j].bearing {
					continue;
				}
				if target_list[k].dist2 >= target_list[j].dist2 {
					continue;
				}
				occluded = true;
				break;
				
			}
			if !occluded {
				curr_targets.push(j);
			}
		}
		for j in 0..curr_targets.len() {
			destroyed_count += 1;
			target_list[curr_targets[j]].destroyed = true;
			if destroyed_count == 200 {
				result_b_destroyed_coords = 100 * target_list[curr_targets[j]].x + target_list[curr_targets[j]].y;
			}
		}
		curr_targets.clear();
		if destroyed_count == target_list.len() {
			break;
		}
	}
	
	println!("Result B: {}", result_b_destroyed_coords);
}