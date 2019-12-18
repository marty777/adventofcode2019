// day 16

// can only be used with a large offset
fn phase_b(input:&mut Vec<i64>, output: &mut Vec<i64>, offset:usize) {
	let mut i:usize = (input.len()) - 1;
	output[i] = input[i];
	i-=1;
	while i >= offset {
		output[i] = (input[i] + output[i+1]) % 10;
		i-=1;
	}
}

fn phase_a(input:&mut Vec<i64>, output: &mut Vec<i64>, offset:usize) {
	let mut i:i64 = (input.len() as i64) - 1;
	while i >= offset as i64 {
		let mut sum:i64 = 0;
		let mut j:i64 = (input.len() as i64) - 1;
		while j >= i{
			let coefficient_pos = (j+1)/(i+1);
			let coefficient:i64;
			if coefficient_pos % 4 == 1 {
				coefficient = 1;
			}
			else if coefficient_pos % 4 == 3 {
				coefficient = -1;
			}
			else {
				coefficient = 0;
			}
			if coefficient == -1 {
				sum -= (*input)[j as usize];
			}
			else if coefficient == 1 {
				sum += (*input)[j as usize];
			}
			j-=1;
		}
		if sum < 0 {
			(*output)[i as usize] = (-sum) % 10;
		}
		else {
			(*output)[i as usize] = sum % 10;
		}
		i-=1;
	}
}


fn print_signal(input:&mut Vec<i64>, offset:usize) {
	for i in offset..offset+8 {
		print!("{}",(*input)[i]);
	}
	println!();
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let mut input:Vec<i64> = Vec::new();
	let mut phase1:Vec<i64> = Vec::new();
	let mut phase2:Vec<i64> = Vec::new();
	for i in 0..vec[0].len() {
		let temp = vec[0][i..i+1].parse::<i64>().unwrap();
		input.push(temp);
		phase1.push(temp);
		phase2.push(temp);
	}
	
	let phases = 100;
	let mut phase_count = 0;
	while phase_count < phases {
		if phase_count % 2 == 0 {
			phase_a(&mut phase1, &mut phase2, 0);
		}
		else {
			phase_a(&mut phase2, &mut phase1, 0);
		}
		phase_count+=1;
	}
	
	print!("Result A: ");
	if phases % 2 == 0 {
		print_signal(&mut phase1, 0);
	}
	else {
		print_signal(&mut phase2, 0);
	}
	
	let mut index:i64 = 6;
	let mut coefficient:usize = 1;
	let mut offset:usize = 0;
	while index >= 0 {
		offset += (input[index as usize] as usize) * coefficient;
		index -=1;
		coefficient *= 10;
	}
	let mut phase3:Vec<i64> = Vec::new();
	let mut phase4:Vec<i64> = Vec::new();
	for _i in 0..10000 {
		for j in 0..input.len() {
			phase3.push(input[j]);
			phase4.push(input[j]);
		}
	}
	phase_count = 0;
	while phase_count < phases {
		if phase_count % 2 == 0 {
			phase_b(&mut phase3, &mut phase4, offset);
		}
		else {
			phase_b(&mut phase4, &mut phase3, offset);
		}
		phase_count+=1;
	}
	print!("Result B: ");
	if phases % 2 == 0 {
		print_signal(&mut phase3, offset);
	}
	else {
		print_signal(&mut phase4, offset);
	}
	
}