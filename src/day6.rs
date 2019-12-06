// day 6 
struct OrbitInfo {
	parent: String,
	child : String,
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut orbit_infos:Vec<OrbitInfo> = Vec::new();
	let mut bodies:Vec<String> = Vec::new();
	for line in vec {
		let body_codes:Vec<&str> = line.split(")").collect();
		orbit_infos.push(OrbitInfo{parent: String::from(body_codes[0]), child: String::from(body_codes[1])});
	}
	
	// build a list of bodies
	for i in 0..orbit_infos.len() {
		let mut found_parent:bool = false;
		let mut found_child:bool = false;
		for j in 0..bodies.len() {
			if bodies[j] == orbit_infos[i].parent {
				found_parent = true;
			}
			if bodies[j] == orbit_infos[i].child {
				found_child = true;
			}
			if found_parent && found_child {
				break;
			}
		}
		if !found_parent  {
			bodies.push(orbit_infos[i].parent.to_string());
		}
		if !found_child  {
			bodies.push(orbit_infos[i].child.to_string());
		}
	}
	
	// build index of child->parent relationship
	let mut parent_index:Vec<i64> = Vec::new();
	for i in 0..bodies.len() {
		if bodies[i] == "COM" {
			parent_index.push(-1);
		}
		else {
			for j in 0..orbit_infos.len() {
				if orbit_infos[j].child == bodies[i] {
					for k in 0..bodies.len() {
						if bodies[k] == orbit_infos[j].parent {
							parent_index.push(k as i64);
							break;
						}
					}
					break;
				}
			}
		}
	}
	
	let mut you_orbits:Vec<i64> = Vec::new();
	let mut san_orbits:Vec<i64> = Vec::new();
	
	let mut orbit_count:u64 = 0;
	for i in 0..bodies.len() {
		let mut curr_index = parent_index[i];
		if curr_index == -1 {
			continue;
		}
		
		let mut trace_you = false;
		let mut trace_san = false;
		if bodies[i] == "YOU" {
			trace_you = true;
		}
		else if bodies[i] == "SAN" {
			trace_san = true;
		}
		
		let mut local_hops = 0;
		loop {
			if trace_you {
				you_orbits.push(curr_index);
			}
			else if trace_san {
				san_orbits.push(curr_index);
			}
			curr_index = parent_index[curr_index as usize];
			local_hops += 1;
			if curr_index == -1 {
				break;
			}
		}
		orbit_count += local_hops;
	}
	
	println!("Result A: {}", orbit_count);
	
	let mut found:bool = false;
	for i in 0..you_orbits.len() {
		for j in 0..san_orbits.len() {
			if you_orbits[i] == san_orbits[j] {
				println!("Result B: {}", i + j);
				found = true;
				break;
			}
		}
		if found {
			break;
		}
	}
	
	if !found {
		println!("Could not find solution path in part B");
	}
}