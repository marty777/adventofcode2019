// day 14

struct Reactant {
	id: usize,
	quantity: u64,
}

struct Reaction {
	inputs:Vec<Reactant>,
	output:Reactant,
}

fn how_much_ore(reactions:&mut Vec<Reaction>, reactant_distances:&mut Vec<i64>, reactant_ids:&mut Vec<&str>, fuel_quantity:u64 )->u64 {
	let mut fuel_id:usize = 0;
	let mut ore_id:usize = 0;
	for i in 0..reactant_ids.len() {
		if (*reactant_ids)[i] == "FUEL" {
			fuel_id = i;
		}
		else if (*reactant_ids)[i] == "ORE" {
			ore_id = i;
		}
		if fuel_id > 0 && ore_id > 0 {// little lazy, should work
			break;
		}
	}
	
	let mut requirements:Vec<Reactant> = Vec::new();
	requirements.push(Reactant{id:fuel_id, quantity:fuel_quantity});
	requirements.push(Reactant{id:ore_id, quantity:0});
	
	loop {
		let mut only_ore:bool = true;
		for i in 0..requirements.len() {
			if requirements[i].id != ore_id {
				only_ore = false;
				break;
			}
		}
		if only_ore {
			//println!("Result A: {}", requirements[0].quantity);
			break;
		}
		
		
		while requirements.len() > 1 {
			
			if requirements.len() == 1 && requirements[0].id == ore_id {
				break;
			}
			
			// determine reactant which has most precursors already in requirements list
			let mut max_requirement_index = 0;
			let mut max_requirements_depth = 0;
			for j in 0..requirements.len() {
				if (*reactant_distances)[requirements[j].id] > max_requirements_depth {
					max_requirement_index = j;
					max_requirements_depth = (*reactant_distances)[requirements[j].id];
				}
			}
			if requirements[max_requirement_index].id == ore_id {
				max_requirement_index = (max_requirement_index + 1) % requirements.len();
				// println!("Trying to skip ore...;");
			}
			
			// determine corresponding reaction
			let mut reaction_index:usize = 0;
			let mut reaction_found = false;
			let required_quantity = requirements[max_requirement_index].quantity;
			for j in 0..(*reactions).len() {
				if (*reactions)[j].output.id == requirements[max_requirement_index].id {
					reaction_index = j;
					reaction_found = true;
				}	
			}
			if !reaction_found {
				println!("error: could not find reaction producing reactant id {}", max_requirement_index);
				break;
			}
			let mut reaction_multiple:u64 = required_quantity/(*reactions)[reaction_index].output.quantity;
			if reaction_multiple * (*reactions)[reaction_index].output.quantity < required_quantity {
				reaction_multiple += 1;
			}
			
			
			requirements.remove(max_requirement_index);
			
			// insert input reactants in required quantities
			for j in 0..(*reactions)[reaction_index].inputs.len() {
				let mut input_found:bool = false;
				for k in 0..requirements.len() {
					if requirements[k].id == (*reactions)[reaction_index].inputs[j].id {
						requirements[k].quantity += (*reactions)[reaction_index].inputs[j].quantity * reaction_multiple;
						input_found = true;
						break;
					}
				}
				if !input_found {
					requirements.push(Reactant{id:(*reactions)[reaction_index].inputs[j].id, quantity:(*reactions)[reaction_index].inputs[j].quantity * reaction_multiple });
				}
			}
			
			break;
		}
	}
	return requirements[0].quantity;
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut reactant_ids:Vec<&str> = Vec::new();
	let mut reactant_distances:Vec<i64> = Vec::new();
	let mut reactions:Vec<Reaction> = Vec::new();
	for line in 0..vec.len() {
		let parts:Vec<&str> = vec[line].split(" => ").collect();
		let part_a:Vec<&str> = parts[0].split(", ").collect();
		let mut output_n:u64 = 0;
		let mut output_s:&str = "";
		for i in 0..parts[1].len() {
			if &parts[1][i..i+1] == " " {
				output_s = &parts[1][i+1..];
				output_n = parts[1][..i].parse::<u64>().unwrap();
				break;
			}
		}
		let mut output_found = false;
		let mut output_id:usize = 0;
		for i in 0..reactant_ids.len() {
			if reactant_ids[i] == output_s {
				output_id = i;
				output_found = true;
				break;
			}
		}
		if !output_found {
			reactant_ids.push(output_s);
			reactant_distances.push(-1);
			output_id = reactant_ids.len() - 1;
		}
		
		let mut reaction:Reaction = Reaction{inputs:Vec::new(), output:Reactant{id:output_id, quantity:output_n}};
		for i in 0..part_a.len() {
			let mut reactant_s:&str = "";
			let mut reactant_n:u64 = 0;
			let mut reactant_id:usize = 0;
			for j in 0..part_a[i].len() {
				if &part_a[i][j..j+1] == " " {
					reactant_s = &part_a[i][j+1..];
					reactant_n = part_a[i][..j].parse::<u64>().unwrap();
					break;
				}
			}
			let mut reactant_found:bool = false;
			for j in 0..reactant_ids.len() {
				if reactant_ids[j] == reactant_s {
					reactant_found = true;
					reactant_id = j;
					break;
				}
			}
			if !reactant_found {
				reactant_ids.push(reactant_s);
				reactant_distances.push(-1);
				reactant_id = reactant_ids.len() - 1;
			}
			reaction.inputs.push(Reactant{id:reactant_id, quantity:reactant_n});
		}
		reactions.push(reaction);
	}
	
	
	let mut fuel_id:usize = 0;
	let mut ore_id:usize = 0;
	for i in 0..reactant_ids.len() {
		if reactant_ids[i] == "FUEL" {
			fuel_id = i;
		}
		else if reactant_ids[i] == "ORE" {
			ore_id = i;
		}
		if fuel_id > 0 && ore_id > 0 {// little lazy, should work
			break;
		}
	}
	
	// determine reactant depth in tree 
	let mut undetermined_reactants:i64 = (reactant_distances.len() - 1) as i64;
	reactant_distances[ore_id] = 0;
	while undetermined_reactants > 0 {
		let mut max_depth = -1;
		for i in 0..reactant_distances.len() {
			if reactant_distances[i] > max_depth {
				max_depth = reactant_distances[i];
			}
		}
		for i in 0..reactant_distances.len() {
			if reactant_distances[i] > -1 {
				continue;
			}
			let mut all_precursors_found = true;
			let mut max_precursors_depth = -1;
			for j in 0..reactions.len() {
				if reactions[j].output.id == i {
					for k in 0..reactions[j].inputs.len() {
						if reactant_distances[reactions[j].inputs[k].id] < 0 {
							all_precursors_found = false;
							break;
						}
						else if reactant_distances[reactions[j].inputs[k].id] > max_precursors_depth{
							max_precursors_depth = reactant_distances[reactions[j].inputs[k].id];
						}
					}
					if all_precursors_found {
						reactant_distances[i] = max_precursors_depth + 1;
						undetermined_reactants -= 1;
					}
				}
			}
		}
	}
	
	let ore = how_much_ore(&mut reactions, &mut reactant_distances, &mut reactant_ids, 1);
	println!("Result a: {}", ore);
	
	// halfassed binary search
	let one_trillion = 1000000000000;
	let mut width = one_trillion;
	let mut guess = 0;
	while width > 10 {
		width = (width)/2;
		let ore = how_much_ore(&mut reactions, &mut reactant_distances, &mut reactant_ids, guess);
		if ore > one_trillion {
			guess -= width;
		}
		else {
			guess += width;
		}
	}
	for i in guess..guess + width + 1 {
		 if how_much_ore(&mut reactions, &mut reactant_distances, &mut reactant_ids, i) > one_trillion {
			 println!("Result B: {}", i -1);
			 break;
		 }
	 }
	
	return;
	
			
}