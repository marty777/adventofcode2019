// day 21

struct Packet {
	address:i64,
	x:i64,
	y:i64,
}

struct Nic {
	prog:super::utility::IntcodeProgram,
	in_buffer:super::utility::IOBuffer,
	out_buffer:super::utility::IOBuffer,
	packet_buffer:Vec<Packet>,
	is_waiting:bool,
	is_exited:bool,
}

fn run_network(prog:&mut super::utility::IntcodeProgram, part_b:bool) {
	let num_nics = 50;
	let idle_detection_cycles = 300;
	let mut idle_detection_count = 0;
	let mut network:Vec<Nic> = Vec::new();
	let mut nat_packet = Packet{address:0, x:0, y:0};
	let mut last_nat_packet = Packet{address:0, x:0, y:0};
	let mut num_nat_packets_delivered = 0;
	// set up each nic and pre-load input with network address
	for i in 0..num_nics {
		let mut nic:Nic = Nic{	prog:super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0}, 
								in_buffer:super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0}, 
								out_buffer:super::utility::IOBuffer{buff:Vec::new(), write_pos:0, read_pos:0},
								packet_buffer:Vec::new(),
								is_waiting:false,
								is_exited:false};
		for i in 0..(*prog).mem.len() {
			nic.prog.mem.push((*prog).mem[i]);
		}
		nic.prog.pos = (*prog).pos;
		nic.prog.relative_base = (*prog).relative_base;
		
		nic.in_buffer.buff.push(i as i64);
		nic.in_buffer.write_pos += 1;
		network.push(nic);
	}
	
	// run each program forward one instruction at a time and note io_waits
	// any io waits will be handled in a second pass after adding any generated
	// packets to queues
	
	loop {
		let mut stopped = 0;
		for i in 0..num_nics {
			if network[i].is_exited {
				stopped += 1;
				continue;
			}
			let mut exit = false;
			let mut io_wait = false;
			let mut error = false;
			let nic = &mut network[i];
			super::utility::intcode_execute_once(&mut (*nic).prog, &mut (*nic).in_buffer, &mut (*nic).out_buffer, &mut exit, &mut io_wait, &mut error);
			if error {
				println!("Error on NIC {}", i);
			}
			if exit || error {
				network[i].is_exited = true;
			}
			if io_wait {
				network[i].is_waiting = true;
			}
			// handle any generated packets
			if network[i].out_buffer.write_pos == 3 {
				let packet = Packet{address: network[i].out_buffer.buff[0], x: network[i].out_buffer.buff[1], y: network[i].out_buffer.buff[2]};
				network[i].out_buffer.buff.clear();
				network[i].out_buffer.write_pos = 0;
				network[i].out_buffer.read_pos = 0;
				if packet.address >= 0 && packet.address < num_nics as i64 {
					network[packet.address as usize].packet_buffer.push(packet);
					idle_detection_count = 0;
				}
				else {
					if packet.address == 255 {
						if !part_b {
							println!("Result A: {}", packet.y);
							return;
						}
						else {
							nat_packet.x = packet.x;
							nat_packet.y = packet.y;
						}
					}
				}
			}
		}
		if stopped == num_nics {
			println!("All NICs stopped");
			break;
		}
		
		let mut num_idle = 0; // using empty packet buffers as a proxy for idleness
		// move packets to io
		for i in 0..num_nics {
			if network[i].packet_buffer.len() == 0 {
				num_idle += 1;
			}
			if network[i].is_waiting {
				if network[i].packet_buffer.len() > 0 {
					let packet = network[i].packet_buffer.remove(0); // FIFO
					network[i].in_buffer.buff.push(packet.x);
					network[i].in_buffer.buff.push(packet.y);
					network[i].in_buffer.write_pos += 2;
				}
				else {
					network[i].in_buffer.buff.push(-1);
					network[i].in_buffer.write_pos += 1;
				}
				network[i].is_waiting = false;
			}
		}
		if num_idle == num_nics {
			idle_detection_count += 1;
		}
		if idle_detection_count == idle_detection_cycles {
			// deliver the nat packet and reset idle detection
			idle_detection_count = 0;
			network[0].packet_buffer.push(Packet{address:0, x:nat_packet.x, y:nat_packet.y});
			if num_nat_packets_delivered > 1 && nat_packet.y == last_nat_packet.y {
				println!("Result B: {}", nat_packet.y);
				return;
			}
			num_nat_packets_delivered += 1;
			last_nat_packet.x = nat_packet.x;
			last_nat_packet.y = nat_packet.y;
		}
		
	}
}

pub fn run(file_path:&str) {
	let vec = super::utility::util_fread(file_path);
	let intcodes_str:Vec<&str> = vec[0].split(",").collect(); 
	let mut prog_a:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	let mut prog_b:super::utility::IntcodeProgram = super::utility::IntcodeProgram{mem:Vec::new(), pos:0, relative_base:0};
	prog_a.mem.reserve(intcodes_str.len());
	prog_b.mem.reserve(intcodes_str.len());
	for code in intcodes_str {
		let temp: i64 = code.parse::<i64>().unwrap();
		prog_a.mem.push(temp);
		prog_b.mem.push(temp);
	}
	
	run_network(&mut prog_a, false);
	run_network(&mut prog_b, true);
	
}