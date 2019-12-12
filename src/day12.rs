// day 12 

struct Moon {
	x:i64,
	y:i64,
	z:i64,
	v_x:i64,
	v_y:i64,
	v_z:i64,
}

struct Acceleration {
	dx:i64,
	dy:i64,
	dz:i64,
}

fn step(moons: &mut Vec<Moon>)->i64 {
	let mut accel:Vec<Acceleration> = Vec::new();
	for _i in 0..(*moons).len() {
		accel.push(Acceleration{dx:0,dy:0,dz:0});
	}
	for i in 0..(*moons).len() {
		for j in 0..(*moons).len() {
			if j == i {
				continue;
			}
			let dx:i64;
			let dy:i64;
			let dz:i64;
			if (*moons)[i].x < (*moons)[j].x {dx = 1;} else if (*moons)[i].x == (*moons)[j].x {dx = 0;} else {dx = -1;}
			if (*moons)[i].y < (*moons)[j].y {dy = 1;} else if (*moons)[i].y == (*moons)[j].y {dy = 0;} else {dy = -1;}
			if (*moons)[i].z < (*moons)[j].z {dz = 1;} else if (*moons)[i].z == (*moons)[j].z {dz = 0;} else {dz = -1;}
			//println!("({} {} {})", dx, dy, dz);
			accel[i].dx += dx;
			accel[i].dy += dy;
			accel[i].dz += dz;
		}
	}

	for i in 0..(*moons).len() {
		(*moons)[i].v_x += accel[i].dx;
		(*moons)[i].v_y += accel[i].dy;
		(*moons)[i].v_z += accel[i].dz;
		(*moons)[i].x += (*moons)[i].v_x;
		(*moons)[i].y += (*moons)[i].v_y;
		(*moons)[i].z += (*moons)[i].v_z;
	}

	// calculate energy
	let mut energy = 0;
	for i in 0..(*moons).len() {
		let potential = (*moons)[i].x.abs() + (*moons)[i].y.abs() + (*moons)[i].z.abs();
		let kinetic = (*moons)[i].v_x.abs() + (*moons)[i].v_y.abs() + (*moons)[i].v_z.abs();
		energy += potential * kinetic;
	}
	return energy;
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut moons:Vec<Moon> = Vec::new();
	let mut moons_start:Vec<Moon> = Vec::new();
	for line in 0..vec.len() {
		let mut index0:usize = 0;
		let mut index1:usize = 0;
		let mut index2:usize = 0;
		let mut index3:usize = 0;
		let mut index4:usize = 0;
		let mut index5:usize = 0;
		for (i,c) in vec[line].chars().enumerate() {
			if c == '=' {
				if index0 == 0 {
					index0 = i;
				}
				else if index2 == 0 {
					index2 = i;
				}
				else if index4 == 0 {
					index4 = i;
				}
			}
			else if c == ',' {
				if index1 == 0 {
					index1 = i;
				}
				else if index3 == 0 {
					index3 = i;
				}
			}
			else if c == '>' {
				index5 = i;
			}
		}
		let x_str = &vec[line][index0+1..index1];
		let y_str = &vec[line][index2+1..index3];
		let z_str = &vec[line][index4+1..index5];
		moons.push(Moon{x:x_str.parse::<i64>().unwrap(), y:y_str.parse::<i64>().unwrap(), z:z_str.parse::<i64>().unwrap(), v_x:0, v_y:0, v_z:0});
		moons_start.push(Moon{x:x_str.parse::<i64>().unwrap(), y:y_str.parse::<i64>().unwrap(), z:z_str.parse::<i64>().unwrap(), v_x:0, v_y:0, v_z:0});
	}
	
	// if there's a period to the system, motion in each dimension is independent of the others. 
	// The overall period of the system will be the mutual least common multiple of the periods 
	// in each dimension. LCM will be obtained using prime factorization of each period.
	let mut period_x = 0;
	let mut period_y = 0;
	let mut period_z = 0;
	let mut step_count = 0;
	loop {
		step_count += 1;
		let energy = step(&mut moons);
		if step_count == 1000 {
			println!("Result A: {}", energy);
		}
		if period_x == 0 {
			let mut x_found = true;
			for j in 0..moons.len() {
				if moons[j].x != moons_start[j].x || moons[j].v_x != moons_start[j].v_x {
					x_found = false;
					break;
				}
			}
			if x_found {
				period_x = step_count;
			}
		}
		if period_y == 0 {
			let mut y_found = true;
			for j in 0..moons.len() {
				if moons[j].y != moons_start[j].y || moons[j].v_y != moons_start[j].v_y {
					y_found = false;
					break;
				}
			}
			if y_found {
				period_y = step_count;
			}
		}
		if period_z == 0 {
			let mut z_found = true;
			for j in 0..moons.len() {
				if moons[j].z != moons_start[j].z || moons[j].v_z != moons_start[j].v_z {
					z_found = false;
					break;
				}
			}
			if z_found {
				period_z = step_count;
			}
		}
		if step_count > 1000 && period_x != 0 && period_y != 0 && period_z != 0 {
			break;
		}
	}
	
	// get prime factors of each period using sieve of eratosthenes
	let mut max = period_x;
	if period_y > max {
		max = period_y;
	}
	if period_z > max {
		max = period_z;
	}

	let mut sieve:Vec<bool> = vec![false;max+1];
	let mut i = 2;
	while i < max+1 {
		let mut j = i;
		loop {
			j += i;
			if j >= max+1 {
				break;
			}
			sieve[j] = true;
		}
		j = i+1;
		while j < max+1 {
			if !sieve[j] {
				break;
			}
			j += 1;
		}
		if j == max+1 {
			break;
		}
		i = j;
	}
	
	let mut factors_x:Vec<usize> = Vec::new();
	let mut factors_y:Vec<usize> = Vec::new();
	let mut factors_z:Vec<usize> = Vec::new();
	
	for i in 2..sieve.len() {
		if sieve[i] {
			continue;
		}
		
		while period_x % i == 0 {
			period_x /= i;
			factors_x.push(i);
		}
		while period_y % i == 0 {
			period_y /= i;
			factors_y.push(i);
		}
		while period_z % i == 0 {
			period_z /= i;
			factors_z.push(i);
		}
		if period_x == 1 && period_y == 1 && period_z == 1 {
			break;
		}
	}
	
	// determine least common multiple of periods using prime factorization
	let mut product = 1;
	loop {
		if factors_x.len() == 0 && factors_y.len() == 0 && factors_z.len() == 0 {
			break;
		}
		let factor;
		if factors_x.len() > 0 {
			factor = factors_x[0];
		}
		else if factors_y.len() > 0 {
			factor = factors_y[0];
		}
		else {
			factor = factors_z[0];
		}
		
		let mut max_count = 0;
		let mut count = 0;
		for j in 0..factors_x.len() {
			if factors_x[j] == factor {
				count+=1;
			}
		}
		if count > max_count {
			max_count = count;
		}
		count = 0;
		for j in 0..factors_y.len() {
			if factors_y[j] == factor {
				count+=1;
			}
		}
		if count > max_count {
			max_count = count;
		}
		count = 0;
		for j in 0..factors_z.len() {
			if factors_z[j] == factor {
				count+=1;
			}
		}
		if count > max_count {
			max_count = count;
		}
		for _j in 0..max_count {
			product *= factor;
		}
		// remove all instances of this factor from each list
		loop {
			let mut found = false;
			for j in 0..factors_x.len() {
				if factors_x[j] == factor {
					found = true;
					factors_x.remove(j);
					break;
				}
			}
			if !found {
				break;
			}
		}
		loop {
			let mut found = false;
			for j in 0..factors_y.len() {
				if factors_y[j] == factor {
					found = true;
					factors_y.remove(j);
					break;
				}
			}
			if !found {
				break;
			}
		}
		loop {
			let mut found = false;
			for j in 0..factors_z.len() {
				if factors_z[j] == factor {
					found = true;
					factors_z.remove(j);
					break;
				}
			}
			if !found {
				break;
			}
		}
	}
	
	println!("Result B: {}", product);
	
}