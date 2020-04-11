// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

fn main(){

	let primes = get_primes(2000000);

	println!("Product of all prime numbers up to 2000000 : {}", primes.iter().sum::<u64>());
}

fn get_primes(ceiling: u32) -> Vec<u64> {
	
	let mut primes:Vec::<u64> = Vec::new();

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
			primes.push(i as u64);
		}
	}

	println!("{} prime numbers found", primes.len());

	return primes;
}