//Smallest positive number that is evenly divisible by all of the numbers from 1 to n
use std::io;

fn main (){

	let mut input = String::new();

	println!("Please input a number");

	io::stdin().read_line(&mut input)
	.expect("Failed to read line");

	let n:u32 = input.trim().parse().unwrap();
	
	let mut primes = get_primes(n);

	println!("Performing prime factorisation...");

	for i in 2..=n{

		let mut current =i;

		for prime in primes.iter_mut(){

			let mut counter = 0;

			while  current % prime.0 == 0{
				current = current/prime.0 ;
				counter += 1;
			}

			if counter > prime.1{
				prime.1 = counter;
			}
		}
	}

	let mut result:u64 = 1;

	for prime in primes.iter_mut(){

		result = result * (prime.0.pow(prime.1) ) as u64
	}

	println!("Smallest positive number evenly divisible by all numbers from 1 to {} : {}", n, result);
}

fn get_primes(ceiling: u32) -> Vec<(u32,u32)> {
	
	let mut primes:Vec::<(u32, u32)> = Vec::new();

	let mut sieve = vec![true; (ceiling + 1)  as usize];
	
	println!("Searching for prime numbers up to {}...", ceiling);

	//Optimisation Potential : Loop only over odd numbers after checking 2
	for i in 2..(ceiling as f64).sqrt() as u32{

		if sieve[i as usize] == true{
			for j in (i*i..=ceiling as u32).step_by(i as usize){

				sieve[j as usize]=false;
			}
		}
	}

	for i in 2..sieve.len(){
		if sieve[i as usize] == true {
			primes.push((i as u32,0));
		}
	}
	
	println!("{} prime numbers found", primes.len());

	return primes;
}