// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn main(){

	let mut primes:Vec<u32> = Vec::new();
	let mut counter =1;

	primes.push(2);

	while primes.len() < 10001 {
		
		counter +=2;
		
		let mut is_prime:bool = true;

		for prime in primes.iter(){
			
			if prime.pow(2)>counter {
				break;
			}
			
			if counter % prime == 0{
				is_prime=false;
			}
		}

		if is_prime{
			primes.push(counter);
		}
	}

	println!("10001th primw number is {}", primes[primes.len()-1]);
}