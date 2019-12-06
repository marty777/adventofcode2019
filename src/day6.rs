// day 6 
struct OrbitInfo {
	parent: String,
	child : String,
}

// could this be vastly faster if I actually built a linked tree structure...who can say?

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut orbit_infos:Vec<OrbitInfo> = Vec::new();
	let mut bodies:Vec<String> = Vec::new();
	for line in vec {
		let body_codes:Vec<&str> = line.split(")").collect();
		orbit_infos.push(OrbitInfo{parent: String::from(body_codes[0]), child: String::from(body_codes[1])});
	}
	
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
	
	bodies.sort();
	
	let mut you_orbits:Vec<String> = Vec::new();
	let mut san_orbits:Vec<String> = Vec::new();
	
	let mut orbit_count:u64 = 0;
	for i in 0..bodies.len() {
		if bodies[i] == "COM" {
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
		
		// trace number of hops back to COM
		let mut local_hops = 0;
		let mut curr_child:String = bodies[i].to_string();
		loop {
			let mut done:bool = false;
			for j in 0..orbit_infos.len() {
				//println!("{} {}", i, j);
				if orbit_infos[j].child == curr_child {
					if orbit_infos[j].parent == "COM" {
						local_hops += 1;
						done = true;
						if trace_san  {
							san_orbits.push("COM".to_string());
						}
						else if trace_you  {
							you_orbits.push("COM".to_string());
						}
						break;
					}
					else {
						local_hops += 1;
						curr_child = orbit_infos[j].parent.to_string();
						if trace_san  {
							san_orbits.push(curr_child.to_string());
						}
						else if trace_you  {
							you_orbits.push(curr_child.to_string());
						}
						break;
					}
				}
			}
			if done {
				orbit_count += local_hops;
				break;
			}
		}
		if i % 100 == 99 {
			println!("Processed orbits of {}/{} bodies. Current total orbits {}...", i+1, bodies.len(), orbit_count);
		}
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
		println!("Could not solution path in part B");
	}
}