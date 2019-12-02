// day 1
pub fn run(file_path:&str) {
		let mut fuel_a:i32 = 0;
		let mut fuel_b:i32 = 0;
		let vec = super::utility::util_fread(file_path);
		for line in vec {
			let mut mass:i32 = line.parse::<i32>().unwrap();
			fuel_a += (mass/3)-2;
			let mut fuelmass:i32 = 0;
			while (mass/3)-2 >= 0 {
				mass = (mass/3)-2;
				fuelmass += mass;
			}
			fuel_b += fuelmass;
		}
		println!("Part A result: {}", fuel_a);
		println!("Part B result: {}", fuel_b);
}