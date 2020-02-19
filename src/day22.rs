// day 22

#[derive(PartialEq)]
enum Technique {
	NewDeck,
	Cut,
	Deal,
}

struct Instruction {
	instruction:Technique,
	argument:i64,
}

// GCD and MMI functions taken from extended euclidean algorithm implementations given at https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
// recursion may be slow 
fn gcd(a:usize, b:usize, x:&mut i64, y:&mut i64)->usize {
    if a == 0 
    { 
        *x = 0;
		*y = 1; 
        return b; 
    } 
	
	let mut x1:i64 = 0;
	let mut y1:i64 = 0;
	let gcd = gcd(b%a, a, &mut x1, &mut y1);
  
    // Update x and y using results of recursive call 
    *x = y1 - ((b/a) as i64) * x1; 
    *y = x1; 
    return gcd; 
}

fn mod_inverse(a:usize, m:usize)->usize {
	let mut x:i64 = 0;
	let mut y:i64 = 0;
	let g = gcd(a,m,&mut x, &mut y);
	if g != 1 {
		return 0;
	}
	else {
		let x1 = positive_modulo(x, m as u64);
		return ( x1 as usize % m ) + m % m;
	}
} 

fn positive_modulo(a:i64, m:u64)->u64 {
	return ((( a % m as i64 ) + m as i64) % m as i64) as u64;
}

fn mod_mul(a:u128, b:u128, m:u128)->u128 {
	let a1 = a % m;
	let b1 = b % m;
	return (a1 * b1) % m;
}

fn run_instructions(input: Vec<String>) {
	let mut instructions:Vec<Instruction> = Vec::new();
	
	let newstack_str = "deal into new stack";
	let deal_str = "deal with increment ";
	let cut_str = "cut ";
	
	for i in 0..input.len() {
		if input[i] == newstack_str {
			instructions.push(Instruction{instruction:Technique::NewDeck, argument: 0});
		}
		else if input[i].len() > deal_str.len() && &input[i][..deal_str.len()] == deal_str {
			let substr = &input[i][deal_str.len()..];
			let arg = substr.trim().parse::<i64>().unwrap();
			instructions.push(Instruction{instruction:Technique::Deal, argument: arg});
		}
		else if input[i].len() > cut_str.len() && &input[i][..cut_str.len()] == cut_str {
			let substr = &input[i][cut_str.len()..];
			let arg = substr.trim().parse::<i64>().unwrap();
			instructions.push(Instruction{instruction:Technique::Cut, argument: arg});
		}
		else {
			println!("Unable to match instruction on line {}", i+1);
			return;
		}
	}
	
	// init deck
	let deck_len = 10007;
	let mut deck:Vec<usize> = Vec::new();
	for i in 0..deck_len {
		deck.push(i);
	}
	
	for i in 0..instructions.len() {
		if instructions[i].instruction == Technique::NewDeck {
			let mut new_stack:Vec<usize> = Vec::new();
			for j in 0..deck.len() {
				new_stack.push(deck[deck.len() - 1 - j]);
			}
			deck.clear();
			for j in 0..new_stack.len() {
				deck.push(new_stack[j]);
			}
		}
		else if instructions[i].instruction == Technique::Cut {
			let mut cut_index = instructions[i].argument % deck_len as i64;
			if cut_index < 0 {
				cut_index += deck_len as i64;
			}
			let mut new_stack:Vec<usize> = Vec::new();
			for j in cut_index as usize..deck.len() {
				new_stack.push(deck[j]);
			}
			for j in 0..cut_index as usize {
				new_stack.push(deck[j]);
			}
			deck.clear();
			for j in 0..new_stack.len() {
				deck.push(new_stack[j]);
			}
		}
		else if instructions[i].instruction == Technique::Deal {
			let mut new_stack:Vec<usize> = Vec::new();
			let increment = instructions[i].argument as usize;
			for _j in 0..deck.len() {
				new_stack.push(0);
			}
			let mut position = 0;
			for j in 0..deck.len() {
				new_stack[position] = deck[j];
				position = (position + increment) % new_stack.len()
			}
			deck.clear();
			for j in 0..new_stack.len() {
				deck.push(new_stack[j]);
			}
		}
	}
	
	for i in 0..deck.len() {
		if deck[i] == 2019 {
			println!("Result A: {}", i);
		}
	}
	
	// PART B this is a mess of casts and poorly named variables, but it works and I'm finished with it.
	
	let deck_len2:i64 = 119315717514047;
	let shuffle_rounds:i64 = 101741582076661;
	
	let position_2020 = 2020;
	let mut offset_term:i64 = 0;
	let mut multiplicative_term:i64 = 1;
	for i in 0..instructions.len() {
		let a;
		let b;
		if instructions[i].instruction == Technique::NewDeck {
			a = -1;
			b = -1;
		}
		else if instructions[i].instruction == Technique::Cut {
			a = 1;
			b = -(instructions[i].argument);
		}
		else {
			a = instructions[i].argument;
			b = 0;
		}
		multiplicative_term = (multiplicative_term * a) % deck_len2 as i64;
		offset_term = ((offset_term * a) + b) % deck_len2 as i64;
	}
	
	let offset_term = positive_modulo(offset_term as i64, deck_len2 as u64);
	
	// now that we have the a and b terms for a single shuffle pass, we need to compute them for shuffle_rounds repetitions
	// from https://codeforces.com/blog/entry/72593, the geometric series for k iterations can be transformed into:
	// F^k(x) = a^k*x + (b * (1 - a^k))/(1-a) % m
	// we can obtain a^k for large k multiplying a in powers of two based on the binary representation of k
	
	let mut a_k_final = 1 as u128;
	let mut a_k = multiplicative_term as u128;
	for i in 0..64 {
		let b = (shuffle_rounds >> i) & 0x01;
		if b == 1 {
			a_k_final = mod_mul(a_k_final, a_k, deck_len2 as u128);
		}
		a_k = mod_mul(a_k, a_k, deck_len2 as u128); // square a_k, i.e. a_k = multiplicative_term^2^i
	}
	
	let mut b_final = positive_modulo(deck_len2 as i64 + 1 - a_k_final as i64, deck_len2 as u64);
	b_final = mod_mul(offset_term as u128, b_final as u128, deck_len2 as u128) as u64;
	let divisor = positive_modulo(deck_len2  + 1 - multiplicative_term, deck_len2 as u64);
	let divisor_inverse = mod_inverse(divisor as usize, deck_len2 as usize);
	b_final = mod_mul(b_final as u128, divisor_inverse as u128, deck_len2 as u128) as u64;
	
	// position x is given by x - b_final / a_k_final
	let x = position_2020;
	let a_k_final_inverse = mod_inverse(a_k_final as usize, deck_len2 as usize);
	let mut result = positive_modulo(x as i64 - b_final as i64, deck_len2 as u64) as u64;
	result = mod_mul(result as u128, a_k_final_inverse as u128, deck_len2 as u128) as u64;

	println!("Result B: {}", result);
	
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	
	run_instructions(vec);
	
	return;
}