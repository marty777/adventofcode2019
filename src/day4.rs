// day 4

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let range_str:Vec<&str> = vec[0].split("-").collect(); 
	let min = range_str[0].parse::<u32>().unwrap();
	let max = range_str[1].parse::<u32>().unwrap();
	
	let mut count_a = 0;
	let mut count_b = 0;
	for i in min..max {
		let a:u32 = i/100000;
		let b:u32 = (i - 100000*a)/10000;
		let c:u32 = (i - 100000*a - 10000*b)/1000;
		let d:u32 = (i - 100000*a - 10000*b - 1000*c)/100;
		let e:u32 = (i - 100000*a - 10000*b - 1000*c - d*100)/10;
		let f:u32 = i - 100000*a - 10000*b - 1000*c - d*100 - 10*e;
		
		if !(a <= b && b <= c && c <= d && d <= e && e <= f) {
			continue;
		}
		if !(a == b || b == c || c == d || d == e || e == f ) {
			continue;
		}
		count_a+=1;
		
		let mut digit_count = 1;
		let mut last_digit = a;
		let mut two_found = false;
		
		if b == last_digit {
			digit_count+=1;
		}
		else {
			if digit_count == 2 {
				two_found = true;
			}
			digit_count = 1;
			last_digit = b;
		}
		if c == last_digit  {
			digit_count+=1;
		}
		else {
			if digit_count == 2 {
				two_found = true;
			}
			digit_count = 1;
			last_digit = c;
		}
		if d == last_digit {
			digit_count+=1;
		}
		else {
			if digit_count == 2 {
				two_found = true;
			}
			digit_count = 1;
			last_digit = d;
		}
		if e == last_digit {
			digit_count+=1;
		}
		else {
			if digit_count == 2 {
				two_found = true;
			}
			digit_count = 1;
			last_digit = e;
		}
		if f == last_digit {
			digit_count+=1;
		}
		else {
			if digit_count == 2 {
				two_found = true;
			}
			digit_count = 1;
		}
		if digit_count == 2 {
			two_found = true;
		}
		
		if two_found {
			count_b+=1;
		}
	}
	
	println!("Result A: {} ", count_a);
	println!("Result B: {} ", count_b);
}