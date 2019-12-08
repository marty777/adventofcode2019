// day 8 
pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let bytes = vec[0].as_bytes();
	let width = 25;
	let height = 6;

	let total_pixels = bytes.len(); 
	let layers = total_pixels/(width*height);
	
	let mut final_bytes:[u8;150] = [2; 150];
	
	let mut min_zeros = width*height;
	let mut result_a = 0;
	for i in 0..layers {
		let mut zero_count = 0;
		let mut one_count = 0;
		let mut two_count = 0;
		for j in i * width * height .. (i+1) * width * height {
			if bytes[j] == 0x30 { // 0
				zero_count += 1;
				if final_bytes[j - (i * width * height)] == 2 {
					final_bytes[j - (i * width * height)] = 0;
				}
			}
			else if bytes[j] == 0x31 { // 1
				one_count += 1;
				if final_bytes[j - (i * width * height)] == 2 {
					final_bytes[j - (i * width * height)] = 1;
				}
			}
			else if bytes[j] == 0x32 { // 2
				two_count += 1;
			}
		}
		if zero_count < min_zeros  {
			min_zeros = zero_count;
			result_a = one_count * two_count;
		}
	}
	println!("Result A: {}", result_a);
	println!("Result B:");
	for y in 0..height {
		for x in 0..width {
			print!("{}",if final_bytes[x + y*width] == 0 {" "} else {"#"});
		}
		println!();
	}
}