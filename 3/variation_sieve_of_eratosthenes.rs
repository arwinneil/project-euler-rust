use std::io;

fn main (){

	let mut input = String::new();

	println!("Please input a number");

	io::stdin().read_line(&mut input)
	.expect("Failed to read line");

	let input:u64 = input.trim().parse().unwrap();

	let mut sieve = vec![true; (input + 1)  as usize];

	//Optimisation Potential : Loop only over odd numbers after checking 2
	for i in 2..(input as f64).sqrt() as u64{

		if sieve[i as usize] == true{
			for j in (i*i..=input).step_by(i as usize){

				sieve[j as usize]=false;
			}
		}
	}

	println!("");

	let mut prime: Vec<u32> = Vec::new();

	for i in 2..sieve.len(){
		if sieve[i as usize] == true {
			
			prime.push(i as u32)
		}
	}

	println!("Prime numbers occuring up to {} inclusive : " , input);
	
	println!("{:#?}",prime);
}