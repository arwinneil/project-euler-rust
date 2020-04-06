use std::io;

fn  main() {

	let mut input = String::new();

	println!("Please input a number");

	io::stdin().read_line(&mut input)
	.expect("Failed to read line");

	let input:u64 = input.trim().parse().unwrap();

	if !(input > 1){
		match input {
			0 => println!("Undefined"),
			1 => println!("The number 1 is called a unit. It has no prime factors and is neither prime nor composite."),
			_ => return
		};
		return;
	}

	let mut largest_prime_factor = 0;
	let mut num = input;

	if num % 2 == 0{
		while num % 2 ==0 {
		num= num/2
	}

	largest_prime_factor = 2;
	
	}
	for i in (3..=((num as f64).sqrt() as u64)).step_by(2) {
		while num % i == 0{
			largest_prime_factor = i ;
			num = num / i ;
		}
	}

	if num > largest_prime_factor{
		largest_prime_factor = num;
	}

		println!("Largest prime factor of {} is : {}", input, largest_prime_factor);
}